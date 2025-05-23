/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::path::PathBuf;

use clap::Parser;
use rocketmq_broker::command::Args;
use rocketmq_broker::Builder;
use rocketmq_common::common::broker::broker_config::BrokerConfig;
use rocketmq_common::EnvUtils::EnvUtils;
use rocketmq_common::ParseConfigFile;
use rocketmq_error::RocketMQResult;
use rocketmq_rust::rocketmq;
use rocketmq_store::config::message_store_config::MessageStoreConfig;
use tracing::info;

#[rocketmq::main]
async fn main() -> RocketMQResult<()> {
    // init logger
    rocketmq_common::log::init_logger_with_level(rocketmq_common::log::Level::INFO);
    let (broker_config, message_store_config) = parse_config_file().unwrap_or_default();
    // boot strap broker
    Builder::new()
        .set_broker_config(broker_config)
        .set_message_store_config(message_store_config)
        .build()
        .boot()
        .await;
    Ok(())
}

fn parse_config_file() -> RocketMQResult<(BrokerConfig, MessageStoreConfig)> {
    let args = Args::parse();
    let home = EnvUtils::get_rocketmq_home();
    info!("Rocketmq(Rust) home: {}", home);
    let config = if let Some(ref config_file) = args.config_file {
        let config_file = PathBuf::from(config_file);
        Ok((
            ParseConfigFile::parse_config_file::<BrokerConfig>(config_file.clone())?,
            ParseConfigFile::parse_config_file::<MessageStoreConfig>(config_file)?,
        ))
    } else {
        let path_buf = PathBuf::from(home.as_str())
            .join("conf")
            .join("broker.toml");
        Ok((
            ParseConfigFile::parse_config_file::<BrokerConfig>(path_buf.clone())?,
            ParseConfigFile::parse_config_file::<MessageStoreConfig>(path_buf)?,
        ))
    };
    config
}
