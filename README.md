# rade package creation tool
Tool to create rade zip packages

# How to use

Create the files needed to create the package: 
```bash
rpkg init
````
Now you can edit the generated config.toml to create a package for rade

```toml:config.toml
exec_name = "exec_name"
install_sh = "install.sh"
exe_file = ""
package_name = ""

```

For example, let's create a package with an executable name of test.

```toml:config.toml
exec_name = "exec_name" # Path of exec_name created earlier with `rpkg init`.
install_sh = "install.sh" # path of install.sh
exe_file = "test" # Name of the executable file. In this case, we assume the name test, so test is used.
package_name = "test" # Package name. # Package name. In this case, it is assumed to be test.
````
Once you have done this, run the following.
````bash
rpkg build
````
This will automatically create the rade package for your environment.

If you are using linux, it should create a file like this
```
test-x86_64-unknown-linux-gnu.radepkg
```
If you want to make it for windows or macos, you can do so by doing the following
```
rpkg build --target macos
```
For windows, just change the macos part!
