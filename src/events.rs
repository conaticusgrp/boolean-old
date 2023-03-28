mod interaction_create;
mod ready;

use crate::log;
use sentry::TransactionContext;
use serenity::model::application::interaction::Interaction;
use serenity::{
    model::prelude::Ready,
    prelude::{Context, EventHandler},
};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let _guard = sentry::Hub::current().push_scope();
        sentry::configure_scope(|scope| {
            scope.set_tag("Event", "ready");
            scope.set_tag("Controller Type", "event");
        });

        let transaction =
            sentry::start_transaction(TransactionContext::new("READY Handler", "function"));
        ready::handle(&ctx, &ready).await;
        transaction.finish();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let _guard = sentry::Hub::current().push_scope();
        sentry::configure_scope(|scope| {
            let other = log::interaction_ctx(&interaction);
            scope.set_context("Interaction", other);
            scope.set_tag("Event", "interaction_create");
            scope.set_tag("Controller Type", "event");
            if let Some(user) = log::interaction_user(&interaction) {
                scope.set_user(Some(user));
            }
        });

        let transaction = sentry::start_transaction(TransactionContext::new(
            "INTERACTION_CREATE Handler",
            "function",
        ));
        interaction_create::handle(&ctx, &interaction).await;
        transaction.finish();
    }
}
