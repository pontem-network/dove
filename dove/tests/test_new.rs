/// Тестирование создания нового проекта
/// dove new demoproject_###
#[cfg(test)]
mod test_dove_cmd {
    use std::io::{Write};
    use termcolor::{ColorChoice, WriteColor, ColorSpec, Color};
    use std::path::{PathBuf, Path};

    /// Создание нового проекта с настройками по умолчанию и сборка его
    /// Имя тестового проекта demoproject_1
    /// cargo run -- new demoproject_1
    /// cargo run -- build -e demoproject_1
    #[test]
    fn create_new_project_with_default_settings(){
        use std::process::Command;

        // Путь до Dove
        let dove_path = get_path_dove().expect("Dove path - not found");

        // Генирация рандомного имени
        let project_name = "demoproject_1".to_string();
        let project_dialect = "pont".to_string();
        let project_address:Option<String> = None;
        print_h1("Dove: New move project. Test-1\n");
        // Шапка с исходными параметрами нового проекта
        print_newproject_settings(&project_name, &project_dialect, &project_address);

        // Поиск существующего проекта с таким именем. Если наден то удалить
        let mut list_projects = get_list_projects();
        if let Some(list) = list_projects.as_ref() {
            // Если найден удалить
            if let Some(finded) = list.iter().find(|it|it.as_os_str().to_str().unwrap_or("").contains(&project_name)){
                print_color_yellow("[WARNING] ");
                print_default(format!("directory exists {}", project_name).as_str());
                print_ln();
                assert_eq!(true, remove_project(finded, &project_name), "[ERROR] remove project {};", project_name);

                // Обновления списка проектов
                list_projects = get_list_projects();
                print_ln();
            }
        }
        // Вывод уже существующих проектов
        print_h2("Existing projects: ");
        print_ln();
        print_projects(&list_projects);

        // =========================================================================================
        // Запуск создания нового проекта
        // =========================================================================================
        print_h2("Create project ");
        let result = Command::new("cargo")
            .args(&["run", "--", "new", &project_name])
            .current_dir(&dove_path)
            .output()
            .map_or_else(|err|{
                    // Неудалось создать новый проект. Вывод сообщения
                    print_ln();
                    print_color_red("[ERROR] ");
                    print_default(format!("cargo run -- new {}", project_name).as_str());
                    print_ln();
                    print_bold("Message: ");
                    print_default(err.to_string().as_str());
                    print_ln();
                    None
                },|result|Some(result));
        assert_ne!(result, None, "failed: cargo run -- new {}", project_name);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            // При создании произошла ошибка
            print_ln();
            print_color_red("[ERROR] ");
            print_default(format!("cargo run -- new {}", project_name).as_str());
            print_ln();
            print_bold("Code: ");
            print_default( result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }

        print_color_green("[SUCCESS]");
        print_ln();
        // =========================================================================================
        // Попытка сборки проекта
        // =========================================================================================
        print_h2("Building project ");
        let result = Command::new("cargo")
            .args(&["run", "--", "build", "-e", &project_name])
            .current_dir(&dove_path)
            .output()
            .map_or_else(|err|{
                // Неудалось создать новый проект. Вывод сообщения
                print_ln();
                print_color_red("[ERROR] ");
                print_default(format!("cargo run -- build -e {}", project_name).as_str());
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },|result|Some(result));

        assert_ne!(result, None, "failed: cargo run -- build -e {}", project_name);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            // При создании произошла ошибка
            print_ln();
            print_color_red("[ERROR] ");
            print_default(format!("cargo run -- build -e {}", project_name).as_str());
            print_ln();
            print_bold("Code: ");
            print_default( result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }

        print_color_green("[SUCCESS]");
        print_ln();

        // Вывод уже существующих проектов
        list_projects = get_list_projects();
        print_ln();
        print_h2("Current list of projects: ");
        print_ln();
        print_projects(&list_projects);

        // Удаление созданного проекта
        if let Some(finded) =  list_projects.as_ref().unwrap().iter().find(|it|it.as_os_str().to_str().unwrap_or("").contains(&project_name)){
            print_ln();
            assert_eq!(true, remove_project(finded, &project_name), "[ERROR] remove project {};", project_name);
        }
    }

    // =============================================================================================
    // Проекты
    // =============================================================================================
    /// Получить путь до dove каталога
    fn get_path_dove()->Option<PathBuf>{
        path_isset_dove("./")
            .or(path_isset_dove("./dove"))
    }
    fn path_isset_dove(path:&str)->Option<PathBuf>{
        Path::new(path)
            .canonicalize()
            .map_or(None, |p|p.to_str().map_or(None,|p|Some(p.to_string())))
            .and_then(|p|{
                if let Some(pos) = (p.clone() + "/").find("/dove/"){
                    let p = {&p[..pos]}.to_string() + "/dove";
                    Some(PathBuf::from(&p))
                } else if let Some(pos) = (p.clone() + "\\").find("\\dove\\") {
                    let p = {&p[..pos]}.to_string() + "\\dove";
                    Some(PathBuf::from(&p))
                }else{
                    None
                }
            })
    }
    /// Получить все созданные проекты
    fn get_list_projects()->Option<Vec<PathBuf>>{
        use std::fs::read_dir;
        let need_folders = vec!["modules","scripts","tests"];
        let need_files = vec!["Dove.toml"];

        get_path_dove()
            .and_then(|folder| read_dir(folder).map_or(None, |resource|Some(resource)) )
            .and_then(|resource|{
                Some(resource
                    .filter_map(|path|path.ok())
                    .map(|path|path.path())
                    .filter(|path|path.is_dir())
                    .filter_map(|path|{
                        read_dir(&path)
                            .map_or(None, |dir|Some(dir))
                            .and_then(|dir|{
                                let finded:Vec<PathBuf> = dir
                                    .filter_map(|p|p.ok())
                                    .map(|p|p.path())
                                    .filter_map(|p|{
                                        let file_name = p.file_name().map_or("",|name|name.to_str().unwrap_or(""));
                                        if ( p.is_dir() && need_folders.contains(&file_name) )
                                            || ( p.is_file() && need_files.contains(&file_name) ) {
                                            Some(p)
                                        }else{
                                            None
                                        }
                                    })
                                    .collect();
                                if finded.len() == need_files.len() + need_folders.len() {
                                    Some(path.clone())
                                }else{
                                    None
                                }
                            })
                    })
                    .collect())
            })
    }
    fn remove_project(path:&PathBuf, project_name:&str)->bool{
        // Удаление директории со всем содержимым
        if let Err(error) = std::fs::remove_dir_all(path) {
            print_color_red("[ERROR] ");
            print_default(format!("remove directory {}", project_name).as_str());
            print_ln();
            print_bold("Message: ");
            print_default(error.to_string().as_str());
            print_ln();
            false
        }else{
            print_color_green("[SUCCESS] ");
            print_default(format!("remove directory {}", project_name).as_str());
            print_ln();
            true
        }
    }
    // =============================================================================================
    // Оформление
    // =============================================================================================
    /// Вывод настроек создоваемого проекта
    fn print_newproject_settings(project_name:&str, project_dialect: &str, project_address:&Option<String>){
        print_h2("New project settings:\n");
        // Название проекта
        print_bold(format!("Project will be created: ").as_str());
        print_reset();
        print_default(format!("{} \n", project_name).as_str());
        // Диалект проекта
        print_bold(format!("Dialect: ").as_str());
        print_reset();
        print_default(format!("{} \n", project_dialect).as_str());
        // Адрес проекта
        print_bold(format!("Address: ").as_str());
        print_reset();
        print_default(format!("{} \n", match project_address.as_ref() {
                Some(path) => path.clone(),
                None => "default".to_string()
            }).as_str());
        print_ln();
    }
    /// Вывод на экран проекта
    fn print_project(project_path:&PathBuf){
        use std::fs::read_to_string;
        use toml::Value;

        let toml_str = read_to_string(project_path.to_str().unwrap().to_string() + "/Dove.toml").unwrap();
        let package = toml_str.parse::<Value>().unwrap().get("package").map_or(toml::Value::String("- NULL -".to_string()),|d|d.clone());
        let project_name = package.get("name")
            .and_then(|v|v.as_str())
            .map_or("- NULL -".to_string(), |v|v.to_string());
        let project_dialect = package.get("dialect")
            .and_then(|v|v.as_str())
            .map_or("- NULL -".to_string(), |v|v.to_string());
        let project_dependencies = package.get("dependencies")
            .and_then(|v| v.get(0) )
            .and_then(|v| v.get("git") )
            .and_then(|v|v.as_str())
            .map_or("- NULL -".to_string(), |v|v.to_string());

        // Заголовок вывода
        print_h3({"Project: ".to_string() + &project_name}.as_str());
        print_ln();

        // Название проекта
        print_bold("Name: ");
        print_default(&project_name);
        print_ln();
        // Диалект проекта
        print_bold("Dialect: ");
        print_default(&project_dialect);
        print_ln();
        // Git проекта
        print_bold("Dependencies: ");
        print_default(&project_dependencies);
        print_ln();

        print_ln();
    }
    /// Вывод на экран списка проектов
    fn print_projects(projects:&Option<Vec<PathBuf>>){
        projects
            .as_ref()
            .map_or_else(||{
                print_default("- empty list -");
            },|projects|{
                projects.iter().for_each(|p|print_project(p));
            });
    }

    fn print_reset(){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.reset().unwrap();
        write!(&mut buffer, "").unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_default(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new()
            .set_bold(false)
            .set_underline(false)).unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_ln(){ print_default("\n"); }

    fn print_h1(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new()
            .set_bold(true)
            .set_underline(true)
        ).unwrap();
        write!(&mut buffer, "{}", text.to_uppercase()).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_h2(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(&mut buffer, "{}", text.to_uppercase()).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_h3(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new().set_underline(true)).unwrap();
        write!(&mut buffer, "{}", text.to_uppercase()).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_bold(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_color_red(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new()
                .set_fg(Some(Color::Red))
            ).unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_color_green(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new()
                .set_fg(Some(Color::Green))
            ).unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
    fn print_color_yellow(text: &str){
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new()
            .set_fg(Some(Color::Yellow))
        ).unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
        // bufwrt.print(&buffer).unwrap();
    }
}