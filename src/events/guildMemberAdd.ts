import {
    GuildMember,
    MessageEmbed,
    PartialGuildMember,
    TextChannel,
} from "discord.js";

import { getSpecialChannel } from "../database";
import { Bot } from "../structures";
import { TypedEvent } from "../types";

export default TypedEvent({
    eventName: "guildMemberAdd",
    run: async (client: Bot, member: GuildMember | PartialGuildMember) => {
        if (member.partial) return;

        const welcomeMessageEmbed = new MessageEmbed()
            .setColor("ORANGE")
            .setTitle("New Member")
            .setDescription(
                `Welcome ${member.user.username} to the conaticus server\n` +
                    "Use `/rolemenu` to choose your pings and languages roles\n" +
                    "Enjoy your stay!"
            );

        const welcomeChannel = await getSpecialChannel(
            member.guild.id,
            "welcomes"
        );

        // This code is only valid in the conaticus server, therefore it can be removed if you are using a fork.
        if (member.guild.id === 949566380523548672) {
            const role = member.guild.roles.cache.find((role) => role.name === "Member");

            member.roles.add(role);
        };

        if (welcomeChannel !== null) {
            const txt = welcomeChannel as TextChannel;
            await txt.send({
                content: `<@${member.user.id}>`,
                embeds: [welcomeMessageEmbed],
            });
        }
    },
});
