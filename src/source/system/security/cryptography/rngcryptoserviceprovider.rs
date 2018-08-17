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

RIDL!{#[uuid(0x2c65d4c0, 0x584c, 0x3e4e, 0x8e, 0x6d, 0x1a, 0xfb, 0x11, 0x2b, 0xff, 0x69)]
interface _RNGCryptoServiceProvider(_RNGCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //RandomNumberGenerator