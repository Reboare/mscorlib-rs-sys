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
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::VARIANT;

use system::reflection::_Type;

RIDL!{#[uuid(0xc8cb1ded, 0x2814, 0x396a, 0x9c, 0xc0, 0x47, 0x3c, 0xa4, 0x97, 0x79, 0xcc)]
interface IFormatProvider(IFormatProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetFormat(
		formatType: *mut  _Type,
		pRetVal: *mut VARIANT,
	) -> HRESULT,
}}