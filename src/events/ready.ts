import { GuildAuditLogs } from "discord.js";
import { getCommandFiles } from "../files";
import { Bot, BotCommand } from "../structures";
import { TypedEvent } from "../types";
import modmailCmds from "../services/modmail";

export default TypedEvent({
    eventName: "ready",
    once: true,
    run: async (client: Bot) => {
        client.logger.console.info(`Logged in as ${client.user?.tag}.`);

        // register our slash commands
        const commandFiles = getCommandFiles();
        const commandArr: BotCommand[] = [
            // NOTE(dylhack): this is a little hack to get modmail up and
            //                working. it's possibly not a preferable way of
            //                doing this.
            ...modmailCmds(),
        ];

        const modules = await Promise.all(
            commandFiles.map((file) => import(file))
        );
        modules.forEach((module) => {
            const command = module.default as BotCommand;
            if (command === undefined) {
                return;
            } else {
                commandArr.push(command);
            }
        });

        await client.register(commandArr);

        // do some audit log stuff
        const tasks: Promise<unknown>[] = [];
        client.guilds.cache.forEach((guild) => {
            const task = guild
                .fetchAuditLogs({
                    type: GuildAuditLogs.Actions.MESSAGE_DELETE,
                    limit: 1,
                })
                .then((audits) => {
                    client.setLastLoggedDeletion(
                        guild.id,
                        audits?.entries.first()
                    );
                })
                .catch(() => null);
            tasks.push(task);
        });
        await Promise.all(tasks);
        client.logger.console.info("Ready");
    },
});
