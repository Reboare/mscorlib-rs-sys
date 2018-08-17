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

RIDL!{#[uuid(0x7499e7e8, 0xdf01, 0x3948, 0xb8, 0xd4, 0xfa, 0x4b, 0x96, 0x61, 0xd3, 0x6b)]
interface _StringComparer(_StringComparerVtbl): IDispatch(IDispatchVtbl)  
{}}
//IComparer, IEqualityComparer, IComparer<string>, IEqualityComparer<string>