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

//! A struct to iterate over parts of a Partition

use crate::error::Result;

use super::{ExecutionPlan, SendableRecordBatchReader};

/// A PartIterator iterates over parts of a partition.
#[derive(Debug)]
pub struct PartIterator<'a> {
    node: &'a dyn ExecutionPlan,
    current_partition: usize,
}

impl<'a> PartIterator<'a> {
    /// creats a new part from an execution plan
    pub fn new(node: &'a dyn ExecutionPlan) -> Self {
        Self {
            node,
            current_partition: 0,
        }
    }
}

impl<'a> Iterator for PartIterator<'a> {
    type Item = Result<SendableRecordBatchReader>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_partition < self.node.output_partitioning().partition_count() {
            self.current_partition += 1;
            Some(self.node.execute(self.current_partition - 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use super::*;
    use crate::physical_plan::expressions::col;
    use crate::physical_plan::{
        csv::{CsvExec, CsvReadOptions},
        projection::ProjectionExec,
    };
    use crate::test;

    #[test]
    fn project_first_column() -> Result<()> {
        let schema = test::aggr_test_schema();

        let partitions = 4;
        let path = test::create_partitioned_csv("aggregate_test_100.csv", partitions)?;

        let csv =
            CsvExec::try_new(&path, CsvReadOptions::new().schema(&schema), None, 1024)?;

        // pick column c1 and name it column c1 in the output schema
        let projection: &dyn ExecutionPlan =
            &ProjectionExec::try_new(vec![(col("c1"), "c1".to_string())], Arc::new(csv))?;

        projection.into_iter().map(|it| {
            let it = it.unwrap();
            it.into_iter().map(|maybe_batch| {
                println!("{:?}", maybe_batch?.num_rows());
                Ok(())
            })
        })
        .flatten()
        .collect::<Result<()>>()?;

        Ok(())
    }
}
