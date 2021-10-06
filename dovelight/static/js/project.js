import * as editor from './editor.js';
import * as cons from './console.js';
import * as localapi from './localapi.js';

/// Create a project object. 
/// It stores information about the project, id, open files
export async function create(id) {
    if (!id) {
        id = null;
    }
    return {
        id: id,
        files: {},
        /// Set project id. All open tabs will be closed
        set_project_id: function(project_id) {
            this.destroy().id = project_id;
            return this;
        },
        /// open file in tab
        open_file: function(file_id, file_name, line, char) {
            if (this.files[file_id]) {
                if (this.files[file_id].set_active) {
                    this.files[file_id]
                        .set_active()
                        .set_position(line, char);
                }
            } else {
                this.files[file_id] = {};
                editor.open_file(this.id, file_id, file_name, line, char)
                    .then(tab => {
                        this.files[file_id] = tab;
                    });
            }

            return this;
        },
        /// closing a file and a tab
        close_file: function(file_id) {
            if (this.files[file_id] && this.files[file_id].destroy) {
                this.files[file_id].destroy();
            }
            delete this.files[file_id];

            let last = this.get_last();
            if (last && last.set_active) {
                last.set_active();
            }

            return this;
        },
        get_last: function() {
            return Object.values(this.files).at(-1);
        },
        get_active_tabs: function() {
            return Object
                .values(this.files)
                .filter(file => {
                    return file.active;
                });
        },
        /// destroy an open project
        destroy: function() {
            /// closing tabs
            for (let index in this.files) {
                if (this.files[index].destroy) {
                    this.files[index].destroy();
                }
            }
            this.files = {};
            this.id = null;
            return this;
        },
        /// building a project
        build: function() {
            if (!this.id) {
                cons.status("Warning: Select a project..");
                return this;
            }
            cons.status("Building a project..");
            let start = Date.now();

            localapi.project_build(this.id)
                .then(response => {
                    cons.status("Done");
                    cons.output("The project was successfully built: " +
                        ((Date.now() - start) / 1000) + "s")
                    console.log(response);
                })
                .catch(err => {
                    cons.status("error when building");
                    cons.output(err);
                });
            return this;
        },
        /// cleaning up the project
        clean: function() {
            if (!this.id) {
                cons.status("Warning: Select a project..");
                return this;
            }
            cons.status("Cleaning up the project..");
            wasm.project_clean(this.id)
                .then(response => {
                    if (response.code == 0) {
                        cons.status("Done")
                        cons.output(response.content);
                    } else {
                        cons.status("Error")
                        console.warn(response);
                    }
                })
                .catch(err => {
                    cons.status("error when cleaning");
                    console.warn(err);
                });
            return this;
        },
        /// testing the project
        test: function() {
            if (!this.id) {
                cons.status("Warning: Select a project..");
                return this;
            }
            cons.status("Testing the project..");
            wasm.project_test(this.id)
                .then(response => {
                    if (response.code == 0) {
                        cons.status("Done")
                        cons.output(response.content);
                    } else {
                        cons.status("Error")
                        console.warn(response);
                    }
                })
                .catch(err => {
                    cons.status("error when testing");
                    console.warn(err);
                });
            return this;
        },
        /// Checking the project
        check: function() {
            if (!this.id) {
                cons.status("Warning: Select a project..");
                return this;
            }
            cons.status("Checking the project..");
            wasm.project_check(this.id)
                .then(response => {
                    if (response.code == 0) {
                        cons.status("Done")
                        cons.output(response.content);
                    } else {
                        cons.status("Error")
                        cons.output(response.content);
                    }
                })
                .catch(err => {
                    cons.status("error when checking");
                    if (err.content) {
                        cons.output(response.content);
                    } else {
                        console.warn(err);
                    }
                });
            return this;
        },
        /// Checking the project
        run_script: function(command) {
            cons.status("Running the script..");
            wasm.dove_run(this.id, command)
                .then(response => {
                    if (response.code == 0) {
                        cons.status("Done")
                        cons.output(response.content);
                    } else {
                        cons.status("Error")
                        cons.output(response.content);
                    }
                })
                .catch(err => {
                    cons.status("Error when running the script");
                    if (err.content) {
                        cons.output(response.content);
                    } else {
                        cons.status("Error: " + err);
                        console.warn(err);
                    }
                });
            return this;
        },
    };
}