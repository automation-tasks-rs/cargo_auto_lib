<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

Includes the plantuml svg generated from the plantuml code.

Search for markers in md files and process plantuml code.  

```markdown
[//comment]: # (auto_plantuml start)

'''plantuml
    @startuml
    [Bob] ..> [Alice]
    @enduml
'''

![svg_534231](images/svg_534231.svg)  

[//comment]: # (auto_plantuml end)
```

In this instructions I changed `[//]` to `[//comment]` and  ticks to single quotes to not process these markers.

Between the last triple backtick and the end marker is the processed svg file.  
Calculate a short hash from the plantuml code.  
If the name of the svg file contains this hash code it means, we already have the correct svg file.  
Else we have to delete the old svg file and get a new one from the modified plantuml code.  
Call the plantuml.com server with the plantuml code and saves the result svg file in folder images/.  
Add the hash code to the filename.

Process plantuml in current directory.  
Finds markers (auto_plantuml start) and (auto_plantuml end) in md files.  
If needed calls the web service and saves the svg file.  
Between markers adds the link to the svg file.  
repo_url like <https://github.com/automation-tasks-rs/sey_currency_converter_pwa>
So the image file link is from the repository and accessible everywhere.

[//]: # (auto_md_to_doc_comments segment end A)
