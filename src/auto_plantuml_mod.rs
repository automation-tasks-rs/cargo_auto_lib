// auto_plantuml_mod.rs

//! finds md files. 
//! Search for plantuml code section:
//! ```plantuml
//! @startuml
//! [Bob] ..> [Alice]
//! @enduml
//! ```
//! 
//! If not exists, adds before and after this lines:
//!  
//! <!-- markdownlint-disable MD033 -->
//! <details>
//! <summary>plantuml code</summary>
//! 
//! ```plantuml
//! @startuml
//! [Bob] ..> [Alice]
//! @enduml
//! ```
//! 
//! </details>  
//! <!-- markdownlint-enable MD033 -->
//! 
//! ![svg_20220425_151400](images/svg_20220425_151400.svg)  
//! 
//! Checks if the svg file exists. Inside the svg is the complete plantuml code.
//! Checks if the code in the md file and in the svg file is equal.
//! If not: Calls the plantuml.com server with the plantuml code and saves the result svg file in folder images/.
//! What will be the name of the svg file? The date/time of the first file creation?
//! The first time it will happen all in one second. Random number? hash?
//! 
//! 
//! 
//! 