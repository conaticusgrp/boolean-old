import { EmbedBuilder, SlashCommandBuilder } from "@discordjs/builders";
import {
    CommandInteraction, Guild,
} from "discord.js";

import { BotCommand, Bot } from "../structures";

class ServerInfo extends BotCommand {
    constructor() {
        super(
            new SlashCommandBuilder()
                .setName("serverinfo")
                .setDescription("Gives you info about our server!")
                .toJSON(),
            { timeout: 6000, requiredPerms: ["SEND_MESSAGES"] }
        );
    }

    public async execute(
        interaction: CommandInteraction<"cached">,
        client: Bot
    ): Promise<void> {

        const serverInfoEmbed = new EmbedBuilder({
            title: "Server Info",

            fields: [
                { name: 'Text Channels', value: client.guild.channels.cache.filter((c) => c.type === 0).toJSON().length, inline: true},
                { name: 'Voice Channels', value: client.guild.channels.cache.filter((c) => c.type === 2).toJSON().length, inline: true},
                { name: 'Categories', value: client.guild.channels.cache.filter((c) => c.type === 4).toJSON().length, inline: true},
                { name: 'Members', value: client.guild.memberCount, inline: true},
                { name: 'Roles', value: client.guild.roles.cache.size, inline: true},
                { name: 'Role List', value: client.guild.roles.cache.toJSON().join(', ') }
            ],

            footer: { text: `Guild ID: ${client.guild.id} | Server Created: ${client.guild.createdAt.toDateString()}`}
        })

        interaction.reply({ embeds: [serverInfoEmbed]})


    }
}

export default new ServerInfo();
