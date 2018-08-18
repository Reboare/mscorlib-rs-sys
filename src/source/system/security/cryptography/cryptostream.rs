// cryptostream.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("8990cb3b-227e-3a43-8264-0057ec763fa0"))
ENUM!{enum CryptoStreamMode
{
    CryptoStreamMode_Read = 0,
    CryptoStreamMode_Write = 1,
}}

RIDL!{#[uuid(0x4134f762, 0xd0ec, 0x3210, 0x93, 0xc0, 0xde, 0x4f, 0x44, 0x3d, 0x56, 0x69)]
interface _CryptoStream(_CryptoStreamVtbl): IDispatch(IDispatchVtbl)  
{}} //Stream, IDisposable