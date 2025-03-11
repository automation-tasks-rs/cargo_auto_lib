<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

New version from date is written to Cargo.toml and service_worker.js.

In Cargo.toml writes the version as the date `yyyy.mmdd.HHMM` ex. `2019.1221.2359`.  
For non-library projects, the semver specification is not really useful.  
Having the version as the date is just fine for executables and much more human readable.  

### service_worker.js

Inside the PWA service worker javascript file is also needed to change the version.  
The program searches for `service_worker.js` and modify the version.  

### no need to change version if no files changed

If src/*.rs or Cargo.toml files are not changed from last compile, than no need to change version.  
The dates of the files will be stored in the file .automation_tasks_rs_file_hashes.json near to Cargo.toml.  
Warning: I don't check if the service worker has changed because it rarely does.  
To know if the projects has changed or not, this function saves the dates of all files into `.automation_tasks_rs_file_hashes.json` near Cargo.toml

[//]: # (auto_md_to_doc_comments segment end A)
