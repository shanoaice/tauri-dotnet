using System.Text;

namespace Tauri.Net;

using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;
using Microsoft.VisualBasic;
using System.Runtime.InteropServices.Marshalling;

/// <summary>
/// This struct is used to pass strings between C# and Rust
/// It is only a bridge, and should not be accessed nor modified directly.
/// Box it using a UnmanagedString class to ensure proper release of pointer
/// through IDisposable and `using` statements.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public struct UnmanagedStringStruct
{
    public unsafe char* chars;
    public nuint length;
    public nuint capacity;
    public unsafe delegate* unmanaged[Cdecl]<char*, nuint, nuint, void> free;
}

public class UnmanagedString(UnmanagedStringStruct stringStruct) : IDisposable
{
    public unsafe char* Chars = stringStruct.chars;
    public nuint Length = stringStruct.length;
    public nuint Capacity = stringStruct.capacity;
    public unsafe delegate* unmanaged[Cdecl]<char*, nuint, nuint, void> Free = stringStruct.free;

    public unsafe ReadOnlySpan<char> AsReadOnlySpan()
    {
        return new ReadOnlySpan<char>(Chars, (int)Length);
    }

    public unsafe Span<char> AsSpan()
    {
        return new Span<char>(Chars, (int)Length);
    }

    public new string ToString()
    {
        return new string(AsReadOnlySpan());
    }

    public unsafe void Dispose()
    {
        Free(Chars, Length, Capacity);
        GC.SuppressFinalize(this);
    }
}

/// <summary>
/// This struct is used to pass strings between C# and Rust
/// It is only a bridge, and should not be accessed nor modified directly.
/// Box it using a UnmanagedString class to ensure proper release of pointer
/// through IDisposable and `using` statements.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public struct UnmanagedRustStringStruct
{
    public unsafe byte* bytes;
    public nuint length;
    public nuint capacity;
    public unsafe delegate* unmanaged[Cdecl]<byte*, nuint, nuint, void> free;
}

public class UnmanagedRustString(UnmanagedRustStringStruct str) : IDisposable
{
    public unsafe byte* Bytes = str.bytes;
    public nuint Length = str.length;
    public nuint Capacity = str.capacity;
    public unsafe delegate* unmanaged[Cdecl]<byte*, nuint, nuint, void> Free = str.free;

    public unsafe void Dispose()
    {
        Free(Bytes, Length, Capacity);
        GC.SuppressFinalize(this);
    }

    public override unsafe string ToString()
    {
        return Encoding.UTF8.GetString(Bytes, (int)Length);
    }
}

[StructLayout(LayoutKind.Sequential)]
public unsafe struct Utf16InteropStringStruct(ushort* chars, int length, delegate* unmanaged[Cdecl]<ushort*, void> free)
{
    public unsafe ushort* Chars = chars;
    public int Length = length;
    public unsafe delegate* unmanaged[Cdecl]<ushort*, void> Free = free;
}

public class Utf16InteropString
{
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)])]
    public static unsafe void FreeMemory(ushort* chars)
    {
        NativeMemory.Free(chars);
    }

    public static unsafe Utf16InteropStringStruct FromString(string str)
    {
        return new Utf16InteropStringStruct(Utf16StringMarshaller.ConvertToUnmanaged(str), str.Length, &FreeMemory);
    }
}
