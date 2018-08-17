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

//enum __declspec(uuid("68db6e95-f774-3ae3-b1de-b0cc80f6e174"))
ENUM!{enum FileOptions
{
    FileOptions_None = 0,
    FileOptions_WriteThrough = 0x80000000,
    FileOptions_Asynchronous = 1073741824,
    FileOptions_RandomAccess = 268435456,
    FileOptions_DeleteOnClose = 67108864,
    FileOptions_SequentialScan = 134217728,
    FileOptions_Encrypted = 16384,
}}