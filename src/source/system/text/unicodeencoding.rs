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

RIDL!{#[uuid(0xf7dd3b7f, 0x2b05, 0x3894, 0x8e, 0xda, 0x59, 0xcd, 0xf9, 0x39, 0x5b, 0x6a)]
interface _UnicodeEncoding(_UnicodeEncodingVtbl): IDispatch(IDispatchVtbl)  
{}} //Encoding