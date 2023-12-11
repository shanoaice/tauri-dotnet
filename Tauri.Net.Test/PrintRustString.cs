namespace Tauri.Net.Test;

using Tauri.Net;

public class PrintRustString
{
    public delegate void PrintStringDelegate(UnmanagedStringStruct rustStringStruct);

    public delegate Utf16InteropStringStruct ReturnStringDelegate();
    
    public static void PrintString (UnmanagedStringStruct rustStringStruct)
    {
        using var rustString = new UnmanagedString(rustStringStruct);
        Console.WriteLine(rustString.ToString());
    }

    public static Utf16InteropStringStruct ReturnString()
    {
        const string str = "Hello Rust!";
        return Utf16InteropString.FromString(str);
    }
}
