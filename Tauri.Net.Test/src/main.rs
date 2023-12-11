use netcorehost::{hostfxr::AssemblyDelegateLoader, nethost, pdcstr};
use tauri_dotnet::ffi::{OwnedUtf16String, UnownedUtf16String, BufferOwnedUtf8String};

fn main() {
    let hostfxr = nethost::load_hostfxr().expect("hostfxr load failed!");
    let context = hostfxr
        .initialize_for_runtime_config(pdcstr!(
            "bin/Debug/net8.0/Tauri.Net.Test.runtimeconfig.json"
        ))
        .expect("hostfxr initialize failed!");
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!("bin/Debug/net8.0/Tauri.Net.Test.dll"))
        .expect("unable to load Test project assembly");

    csharp_print_unmanaged_string(&delegate_loader, "Hello, World!".into());
}

fn csharp_print_unmanaged_string(delegate_loader: &AssemblyDelegateLoader, str: String) {
    let print_string = delegate_loader
        .get_function::<fn(OwnedUtf16String)>(
            pdcstr!("Tauri.Net.Test.PrintRustString, Tauri.Net.Test"),
            pdcstr!("PrintString"),
            pdcstr!("Tauri.Net.Test.PrintRustString+PrintStringDelegate, Tauri.Net.Test"),
        )
        .unwrap();

    let return_string = delegate_loader
        .get_function::<fn() -> UnownedUtf16String>(
            pdcstr!("Tauri.Net.Test.PrintRustString, Tauri.Net.Test"),
            pdcstr!("ReturnString"),
            pdcstr!("Tauri.Net.Test.PrintRustString+ReturnStringDelegate, Tauri.Net.Test"),
        )
        .unwrap();

        let return_rust_owned_buffer_string = delegate_loader
        .get_function::<fn(extern "C" fn(i32) -> BufferOwnedUtf8String) -> BufferOwnedUtf8String>(
            pdcstr!("Tauri.Net.Test.PrintRustString, Tauri.Net.Test"),
            pdcstr!("ReturnUnownedString"),
            pdcstr!("Tauri.Net.Test.PrintRustString+ReturnUnownedStringDelegate, Tauri.Net.Test"),
        )
        .unwrap();

    let owned_str = tauri_dotnet::ffi::OwnedUtf16String::from(str);
    print_string(owned_str);

    println!("{}", String::from(return_string()));

    println!("{}", String::from(return_rust_owned_buffer_string(BufferOwnedUtf8String::new)));
}
