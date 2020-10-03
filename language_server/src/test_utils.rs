use utils::MoveFile;
use crate::inner::config::Config;
use crate::global_state::{GlobalStateSnapshot, initialize_new_global_state};
use crate::inner::change::AnalysisChange;
use utils::io::read_move_files;

pub fn global_state_snapshot(
    file: MoveFile,
    config: Config,
    additional_files: Vec<MoveFile>,
) -> GlobalStateSnapshot {
    let mut global_state = initialize_new_global_state(config);
    let mut change = AnalysisChange::new();

    for folder in &global_state.config().modules_folders {
        for (fpath, text) in read_move_files(folder) {
            change.add_file(fpath, text);
        }
    }

    for (fpath, text) in additional_files {
        change.add_file(fpath, text);
    }
    change.update_file(file.0, file.1);

    global_state.apply_change(change);
    global_state.snapshot()
}
