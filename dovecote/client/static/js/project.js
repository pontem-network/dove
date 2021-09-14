import * as editor from './editor.js';

/// Create a project object. 
/// It stores information about the project, id, open files
export async function create(id) {
    if(!id) { id=null; }
    return {
        id:id,
        files:{},
        /// Set project id. All open tabs will be closed
        set_project_id: function(project_id) {
            this.destroy().id = project_id;
            return this;
        },
        /// open file in tab
        open_file: function(file_id, file_name) {
            if(this.files[file_id]) {
                if(this.files[file_id].active) {
                    this.files[file_id].active();
                }
            } else {
                this.files[file_id]={};
                editor.open_file(this.id, file_id, file_name)
                    .then(tab => {
                        this.files[file_id] = tab;
                    });                
            }
            
            return this;
        },
        /// closing a file and a tab
        close_file: function(file_id) {
            if(this.files[file_id] && this.files[file_id].destroy) {
                this.files[file_id].destroy();
            }
            delete this.files[file_id];
            
            return this;
        },
        /// destroy an open project
        destroy: function() {
            /// closing tabs
            for(let index in this.files) {
                if(this.files[index].destroy) {
                    this.files[index].destroy();
                }
            }
            this.files={};
            this.id=null;
            return this;
        }
    };
}


