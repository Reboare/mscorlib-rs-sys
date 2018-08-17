//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.


use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x7ae4b03c, 0x414a, 0x36e0, 0xba, 0x68, 0xf9, 0x60, 0x30, 0x04, 0xc9, 0x25)]
interface _RandomNumberGenerator(_RandomNumberGeneratorVtbl): IDispatch(IDispatchVtbl)  
{}} //abstract, IDisposable
