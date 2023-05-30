# rmall.rs
A universal CLI that (poorly) attempts to replicate the behavior of the [gnu `rm` utility](https://man7.org/linux/man-pages/man1/rm.1.html). rmall is a combination of `rm` and the word "all" which refers to how this CLI supports multiple arguments. 
> ⚠️ **_Disclaimer_** ⚠️  
> If this tool does something unexpected, I am not responsible. rmall is not guaranteed to work perfectly. I only made it to make certain tasks easier for myself.
## Download & Use
You need to have rust installed. I don't know what the MSRV is for this. 
```sh
cargo install rmall
```  
and then...
```sh
rmall -rf <insert file(s) or folder(s)>
```
