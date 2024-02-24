# auto_plantuml

Search for markers in md files and process plantuml code.  

```markdown
[comment]: # (auto_plantuml start)

'''plantuml
    @startuml
    [Bob] ..> [Alice]
    @enduml
'''

![svg_534231](images/svg_534231.svg)  

[comment]: # (auto_plantuml end)
```

In your markdown, change the word `[comment]` with double slash `[//]`. And single quotes with ticks.

Between the last triple backtick and the end marker is the processed svg file.  
Calculate a short hash from the plantuml code.  
If the name of the svg file contains this hash code it means, we already have the correct svg file.  
Else we have to delete the old svg file and get a new one from the modified plantuml code.  
Call the plantuml.com server with the plantuml code and saves the result svg file in folder images/.  
Add the hash code to the filename.

process plantuml in current directory
finds markers (auto_plantuml start) and (auto_plantuml end) in md files
if needed calls the web service and saves the svg file
Between markers adds the link to the svg file
repo_url like <https://github.com/bestia-dev/sey_currency_converter_pwa>
so the image file link is from the repository and accessible everywhere