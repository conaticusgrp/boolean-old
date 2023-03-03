import { Client, Collection, GuildAuditLogsEntry, Intents } from "discord.js";

import { getEventFiles } from "../files";
import { BotCommand, Logger } from "../structures";
import { IBotEvent } from "../types";

export default class Bot extends Client<true> {
    // eslint-disable-next-line no-use-before-define
    protected static instance: Bot;

    public commands = new Collection<string, BotCommand>();

    public logger = new Logger({ level: process.env.LOG_LEVEL || "info" });

    // NOTE(HordLawk): This feels wrong, but I don't know TS and I need to
    //                 use this property
    // NOTE(hayper): I got you fam
    private lastLoggedDeletion: Map<
        string,
        GuildAuditLogsEntry<"MESSAGE_DELETE">
    >;

    constructor() {
        super({
            intents: [
                Intents.FLAGS.GUILDS,
                Intents.FLAGS.GUILD_MESSAGES,
                Intents.FLAGS.GUILD_MESSAGE_REACTIONS,
                Intents.FLAGS.GUILD_MEMBERS,
                Intents.FLAGS.GUILD_PRESENCES,
            ],
            partials: ["MESSAGE", "CHANNEL", "REACTION"],
        });
        this.lastLoggedDeletion = new Map();
        Bot.instance = this;
    }

    static getInstance(): Bot {
        return Bot.instance;
    }

    getLastLoggedDeletion(
        guildId: string
    ): GuildAuditLogsEntry<"MESSAGE_DELETE"> | null {
        return this.lastLoggedDeletion.get(guildId) || null;
    }

    setLastLoggedDeletion(
        guildId: string,
        value?: GuildAuditLogsEntry<"MESSAGE_DELETE">
    ) {
        // NOTE(dylhack): this allows for shorter syntax from outside usage.
        if (value !== undefined) {
            this.lastLoggedDeletion.set(guildId, value);
        }
    }

    async start() {
        await this.initModules();
        await this.login(process.env.TOKEN || "");
    }

    async initModules() {
        const tasks: Promise<unknown>[] = [];
        const eventFiles = getEventFiles();
        const modules = await Promise.all(
            eventFiles.map((file) => import(file))
        );
        modules.forEach((module) => {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const event = module.default as IBotEvent<any>;
            if (!event) {
                return;
            } else {
                if (event.once) {
                    this.once(event.eventName, event.run.bind(null, this));
                } else {
                    this.on(event.eventName, event.run.bind(null, this));
                }
                this.logger.console.debug(
                    `Registered event ${event.eventName}`
                );
            }
        });

        await Promise.all(tasks);
        this.logger.console.info("Registering slash commands");
    }

    async register(cmds: BotCommand[]): Promise<void> {
        // Register to a testing server
        const payload = cmds.map((cmd) => {
            this.commands.set(cmd.data.name, cmd);
            return cmd.data;
        });
        const devServer = process.env.DEV_SERVER;
        if (devServer !== undefined) {
            const guild = await this.guilds.fetch(devServer);
            await guild.commands.set(payload);
            this.logger.console.info(`Registered commands to ${devServer}`);
            return;
        }
        // else... register globally

        // clear dev commands
        const tasks: Promise<unknown>[] = [];
        this.guilds.cache.forEach((guild) => {
            const task = guild.commands.set([]);
            tasks.push(task);
        });
        await Promise.all(tasks).catch(() => null);
        // register global commands
        await this.application.commands.set(payload);
        this.logger.console.info("Registered commands globally");
    }
}
