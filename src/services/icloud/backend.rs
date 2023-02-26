// Copyright 2023 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;

use crate::raw::*;
use crate::*;

pub struct IcloudBuilder {
    root: Option<String>,
}

pub struct IcloudBackend {
    root: String,

    client: HttpClient,
}

#[async_trait]
impl Accessor for IcloudBackend {
    type Reader = IncomingAsyncBody;
    type BlockingReader = ();
    type Pager = ();
    type BlockingPager = ();

    fn metadata(&self) -> AccessorMetadata {
        let mut ma = AccessorMetadata::default();
        ma.set_scheme(Scheme::Icloud)
            .set_root(&self.root)
            .set_capabilities(AccessorCapability::Read | AccessorCapability::Write)
            .set_hints(AccessorHint::ReadStreamable);

        ma
    }
}

impl IcloudBackend {}
