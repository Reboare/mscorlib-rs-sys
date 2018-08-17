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

use winapi::ctypes::c_long;
//struct __declspec(uuid("94942670-4acf-3572-92d1-0916cd777e00"))
STRUCT!{struct TimeSpan {
    _ticks: c_long,
}}
//IComparable, IComparable<TimeSpan>, IEquatable<TimeSpan>, IFormattable