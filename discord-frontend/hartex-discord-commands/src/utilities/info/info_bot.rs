/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use std::time::SystemTime;

use hartex_backend_models::Response;
use hartex_backend_models_v2::uptime::UptimeQuery;
use hartex_backend_models_v2::uptime::UptimeResponse;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::tokio::net::TcpStream;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use hartex_log::log;
use http_body_util::BodyExt;
use hyper::body::Buf;
use hyper::client::conn::http2::handshake;
use hyper::header::ACCEPT;
use hyper::header::USER_AGENT;
use hyper::Method;
use hyper::Request;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;

pub async fn execute(interaction: Interaction, _: CommandDataOption) -> miette::Result<()> {
    let interaction_client = CLIENT.interaction(interaction.application_id);
    let locale = interaction.locale.unwrap_or_else(|| String::from("en-GB"));
    let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

    let stream = TcpStream::connect("https://discord.com")
        .await
        .into_diagnostic()?;
    let (mut sender, _) = handshake(TokioExecutor::new(), TokioIo::new(stream))
        .await
        .into_diagnostic()?;

    let api_domain = env::var("API_DOMAIN").into_diagnostic()?;
    let uri = format!("http://{api_domain}/api/v2/uptime");
    let now = SystemTime::now();

    log::debug!("sending a request to {}", &uri);

    let query = UptimeQuery::new("HarTex Nightly");
    let request = Request::builder()
        .uri(uri)
        .method(Method::POST)
        .header(ACCEPT, "application/json")
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.1.0) DiscordFrontend",
        )
        .body(serde_json::to_string(&query).into_diagnostic()?)
        .into_diagnostic()?;

    let result = sender.send_request(request).await.into_diagnostic()?;
    let body = result.collect().await.into_diagnostic()?.aggregate();
    let response: Response<UptimeResponse> =
        serde_json::from_reader(body.reader()).into_diagnostic()?;

    let latency = now.elapsed().into_diagnostic()?.as_millis();

    let data = response.data();
    let timestamp = data.start_timestamp();

    let botinfo_embed_botstarted_field_name =
        localizer.utilities_plugin_botinfo_embed_botstarted_field_name()?;
    let botinfo_embed_latency_field_name =
        localizer.utilities_plugin_botinfo_embed_latency_field_name()?;
    let botinfo_embed_title = localizer.utilities_plugin_botinfo_embed_title()?;

    let embed = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            botinfo_embed_botstarted_field_name,
            timestamp.to_string().discord_relative_timestamp(),
        ))
        .field(EmbedFieldBuilder::new(
            botinfo_embed_latency_field_name,
            latency.to_string().discord_inline_code(),
        ))
        .title(botinfo_embed_title)
        .validate()
        .into_diagnostic()?
        .build();

    interaction_client
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .embeds(vec![embed])
                        .build(),
                ),
            },
        )
        .await
        .into_diagnostic()?;

    Ok(())
}
