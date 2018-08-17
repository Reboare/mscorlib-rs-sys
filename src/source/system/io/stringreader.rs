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
RIDL!{#[uuid(0x59733b03, 0x0ea5, 0x358c, 0x95, 0xb5, 0x65, 0x9f, 0xcd, 0x9a, 0xa0, 0xb4)]
interface _StringReader(_StringReaderVtbl): IDispatch(IDispatchVtbl)  
{}}