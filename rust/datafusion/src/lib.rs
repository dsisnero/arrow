// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! DataFusion is a modern distributed compute platform implemented in Rust that uses
//! Apache Arrow as the memory model

extern crate arrow;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sqlparser;

pub mod datasource;
pub mod dfparser;
pub mod execution;
pub mod logicalplan;
pub mod optimizer;
pub mod sqlplanner;
