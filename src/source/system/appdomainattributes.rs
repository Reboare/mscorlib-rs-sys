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

//enum __declspec(uuid("8a6c24c5-1f87-37c2-bc4d-3421eb62d4c1"))
ENUM!{enum LoaderOptimization {
    LoaderOptimization_NotSpecified = 0,
    LoaderOptimization_SingleDomain = 1,
    LoaderOptimization_MultiDomain = 2,
    LoaderOptimization_MultiDomainHost = 3,
    LoaderOptimization_DomainMask = 3,
    LoaderOptimization_DisallowBindings = 4,
}}

RIDL!{#[uuid(0xce59d7ad, 0x05ca, 0x33b4, 0xa1, 0xdd, 0x06, 0x02, 0x8d, 0x46, 0xe9, 0xd2)]
interface _LoaderOptimizationAttribute(_LoaderOptimizationAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}
