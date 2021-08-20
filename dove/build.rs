use anyhow::anyhow;
use anyhow::Error;
use git2::{Commit, Direction, IndexAddOption, ObjectType, Repository, Signature};
use std::fs::{create_dir_all, remove_dir_all, read_to_string};
use std::path::PathBuf;
use fs_extra::file::write_all;

fn main() {
    create_test_repository();
}

fn create_test_repository() {
    if get_path_target(None).join("test.git").exists() {
        return;
    }

    git2::Repository::init_bare(&get_path_git()).unwrap();

    create_branch("master", true, None);
    create_branch("for_tag_v1", false, Some("v1"));
    create_branch("for_tag_v2", false, Some("v2"));
    create_branch("path", false, None);
    create_branch("script_main", false, None);
    create_branch("test_rec", false, None);
    create_branch("without_dove_toml", false, None);
    create_branch("no_dove_toml", false, None);
    create_branch("unnecessary_elements", false, None);

    if remove_dir_all(&get_path_projectgit()).is_err() {
        // windows does not allow you to delete a directory
    }
}

fn get_path_target(sub: Option<&str>) -> PathBuf {
    let path = if let Some(sub) = sub {
        get_path_target(None).join(sub)
    } else {
        PathBuf::from("../target")
    };

    if !path.exists() {
        create_dir_all(&path).unwrap();
    }
    path.canonicalize().unwrap()
}

fn get_path_git() -> PathBuf {
    get_path_target(Some("test.git"))
}

fn get_path_projectgit() -> PathBuf {
    get_path_target(Some("test_git"))
}

fn get_path_template(name: &str) -> PathBuf {
    PathBuf::from("resources").join("for_git").join(name)
}

fn copy_template(name: &str) {
    let to_path = get_path_projectgit();
    fs_extra::copy_items(
        &get_path_template(name)
            .read_dir()
            .unwrap()
            .map(|item| item.map(|item| item.path()))
            .collect::<Result<Vec<PathBuf>, _>>()
            .unwrap(),
        &to_path,
        &fs_extra::dir::CopyOptions::new(),
    )
    .unwrap();

    let path_dovetoml = to_path.join("Dove.toml");
    if path_dovetoml.exists() {
        let mut cont = read_to_string(&path_dovetoml).unwrap();
        let mut save = false;
        if cont.contains("###test_move_project###") {
            save = true;
            cont = cont.replace(
                "###test_move_project###",
                &PathBuf::from("resources/test_move_project")
                    .canonicalize()
                    .unwrap()
                    .display()
                    .to_string(),
            );
        }
        if cont.contains("###current_git###") {
            save = true;
            cont = cont.replace("###current_git###", &get_path_git().display().to_string());
        }
        if save {
            write_all(&path_dovetoml, &cont).unwrap();
        }
    }
}

fn clear_project_directory() {
    let list = get_path_projectgit()
        .read_dir()
        .unwrap()
        .map(|de| de.map(|de| de.path()))
        .filter(|path| {
            if let Ok(p) = path {
                p.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| !name.starts_with('.'))
                    .unwrap_or(false)
            } else {
                false
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    fs_extra::remove_items(&list).unwrap();
}

fn create_branch(name: &str, first: bool, tag: Option<&str>) {
    let repo_project = if first {
        Repository::clone(
            &get_path_git().as_os_str().to_str().unwrap().to_string(),
            get_path_projectgit(),
        )
        .unwrap()
    } else {
        Repository::open(get_path_projectgit()).unwrap()
    };
    clear_project_directory();
    copy_template(name);

    let mut index = repo_project.index().unwrap();
    index.remove_all(&["*"], None).unwrap();
    index
        .add_all(&["*"], IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();

    let oid = index.write_tree().unwrap();
    let signature = Signature::now("VMM", "maxim@pontem.network").unwrap();
    let tree = repo_project.find_tree(oid).unwrap();

    let new_commit = if first {
        repo_project
            .commit(
                Some("HEAD"),                          //  point HEAD to our new commit
                &signature,                            // author
                &signature,                            // committer
                &format!("branch for test: {}", name), // commit message
                &tree,                                 // tree
                &[],
            )
            .unwrap()
    } else {
        repo_project
            .commit(
                Some(&format!("refs/heads/{0}", name)), //  point HEAD to our new commit
                &signature,                             // author
                &signature,                             // committer
                &format!("branch for test: {}", name),  // commit message
                &tree,                                  // tree
                &[&get_last_commite(&repo_project).unwrap()],
            )
            .unwrap()
    };

    repo_project
        .remote_add_push(name, &format!("refs/heads/{0}", name))
        .unwrap();

    let mut remote = repo_project.find_remote("origin").unwrap();
    remote.connect(Direction::Push).unwrap();

    if let Some(tag) = tag {
        repo_project
            .tag(
                tag,
                &repo_project.find_object(new_commit, None).unwrap(),
                &signature,
                &format!("tag {}", tag),
                false,
            )
            .unwrap();
        remote
            .push(
                &[
                    format!("refs/heads/{0}", name),
                    format!("refs/tags/{0}", tag),
                ],
                None,
            )
            .unwrap();
    } else {
        remote
            .push(&[format!("refs/heads/{0}", name)], None)
            .unwrap();
    }
}

fn get_last_commite(repo_project: &Repository) -> Result<Commit, Error> {
    repo_project
        .head()?
        .resolve()?
        .peel(ObjectType::Commit)?
        .into_commit()
        .map_err(|err| anyhow!("{:?}", err))
}
