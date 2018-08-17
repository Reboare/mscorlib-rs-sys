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

//enum __declspec(uuid("28ee6224-fd72-3bdf-b248-ba9102fceb14"))
ENUM!{enum TypeAttributes {
    TypeAttributes_VisibilityMask = 7,
    TypeAttributes_NotPublic = 0,
    TypeAttributes_Public = 1,
    TypeAttributes_NestedPublic = 2,
    TypeAttributes_NestedPrivate = 3,
    TypeAttributes_NestedFamily = 4,
    TypeAttributes_NestedAssembly = 5,
    TypeAttributes_NestedFamANDAssem = 6,
    TypeAttributes_NestedFamORAssem = 7,
    TypeAttributes_LayoutMask = 24,
    TypeAttributes_AutoLayout = 0,
    TypeAttributes_SequentialLayout = 8,
    TypeAttributes_ExplicitLayout = 16,
    TypeAttributes_ClassSemanticsMask = 32,
    TypeAttributes_Class = 0,
    TypeAttributes_Interface = 32,
    TypeAttributes_Abstract = 128,
    TypeAttributes_Sealed = 256,
    TypeAttributes_SpecialName = 1024,
    TypeAttributes_Import = 4096,
    TypeAttributes_Serializable = 8192,
    TypeAttributes_StringFormatMask = 196608,
    TypeAttributes_AnsiClass = 0,
    TypeAttributes_UnicodeClass = 65536,
    TypeAttributes_AutoClass = 131072,
    TypeAttributes_CustomFormatClass = 196608,
    TypeAttributes_CustomFormatMask = 12582912,
    TypeAttributes_BeforeFieldInit = 1048576,
    TypeAttributes_ReservedMask = 264192,
    TypeAttributes_RTSpecialName = 2048,
    TypeAttributes_HasSecurity = 262144,
}}
