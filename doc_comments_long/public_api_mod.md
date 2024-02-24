# public_api_mod

The Public API of a library is a pain in the a...  
Every time I modify something I have to think how it will affect the users of the library.  
They could have tens or hundreds of places where they use the library. Breaking changes are terrible things.  
The developers are not willing to change their code every time a library changes slightly.  
Yes, there is the semver to show if the new library is API compatible. That helps a lot.  
It is dreaded if the first version of a function does not return a Result<>.  
Then later we will surely come to the point, that we need to return a Result<>. This is a terrible breaking change.  
It is wise to return a Result always. Even when that is not needed right now. It will surely be needed in the future.  
Another tactic is to make new functions with a different name and ornament the old functions as Obsolete.

This library is used by the automation_tasks_rs executable.  
I want to have here the complete and exact definition of the public API.  
Therefore I will not use reexports like `pub use` or `pub mod`.  
This way I can always know easily if my public API has changed.  
Just compare the lib.rs file in git.  
Adding functions, structs, methods and enums is ok, it does not break the Public API.  
But modifying existing functions, methods or enums will break the compatibility.  
AFTERTHOUGHT: This is a very time-consuming process to do manually.  
Should use a utility, but that app is complicated to create. It must understand the Rust code.