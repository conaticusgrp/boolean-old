-------------------------------------------------------------------------------
-- Server Configuration                                                      --
-------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS guild_config (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  id VARCHAR(36) NOT NULL,
  guild_id TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS badge (
  id VARCHAR(36) NOT NULL,
  emoji TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS special_channel (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  id VARCHAR(36) NOT NULL,
  config_id VARCHAR(36) NOT NULL,
  label VARCHAR(255) NOT NULL,
  channel_id TEXT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (config_id) REFERENCES guild_config(id)
);

CREATE TABLE IF NOT EXISTS special_role (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  id VARCHAR(36) NOT NULL,
  config_id VARCHAR(36) NOT NULL,
  label VARCHAR(255) NOT NULL,
  role_id TEXT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (config_id) REFERENCES guild_config(id)
);



-------------------------------------------------------------------------------
-- Modmail Feature                                                           --
-------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS modmail (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  id VARCHAR(36) NOT NULL,
  guild_id VARCHAR(255) NOT NULL,
  channel_id VARCHAR(255) NOT NULL,
  member_id VARCHAR(255) NOT NULL,
  author_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS modmail_message (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  id VARCHAR(36) NOT NULL,
  guild_id VARCHAR(255) NOT NULL,
  channel_id VARCHAR(255),
  content TEXT NOT NULL,
  modmail_id VARCHAR(255) NOT NULL,
  sender_id VARCHAR(255) NOT NULL,
  member_copy_id VARCHAR(255) NOT NULL,
  staff_copy_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (modmail_id) REFERENCES modmail(id)
);

CREATE TABLE IF NOT EXISTS modmail_attachment (
  id VARCHAR(36) NOT NULL,
  message_id VARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (message_id) REFERENCES modmail_message(id)
);

CREATE TABLE IF NOT EXISTS modmail_edit (
  id VARCHAR(36) NOT NULL,
  message_id VARCHAR(255) NOT NULL,
  content TEXT NOT NULL,
  iteration INT NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (message_id) REFERENCES modmail_message(id)
);


-------------------------------------------------------------------------------
-- Role Menus Feature                                                        --
-------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS role_menu_list (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  id VARCHAR(36) NOT NULL,
  guild_id TEXT NOT NULL,
  title VARCHAR(255) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS role_menu_item (
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  role_id TEXT NOT NULL,
  list_id VARCHAR(36) NOT NULL,
  id VARCHAR(36) NOT NULL,
  PRIMARY KEY (role_id),
  FOREIGN KEY (list_id) REFERENCES role_menu_list(id)
);

INSERT INTO badge (id, emoji) VALUES
  ('DISCORD_EMPLOYEE', '<:discord_staff:585598614521511948>'),
  ('PARTNERED_SERVER_OWNER', '<:discord_partner:585598614685089792>'),
  ('HYPESQUAD_EVENTS', '<:discord_hypesquad:971698541313556491>'),
  ('BUGHUNTER_LEVEL_1', '<:discord_bughunterlv1:971698294743007253>'),
  ('BUGHUNTER_LEVEL_2', '<:discord_bughunterlv2:971698415438274570>'),
  ('HOUSE_BRAVERY', '<:bravery:889966063100493914>'),
  ('HOUSE_BRILLIANCE', '<:brilliance:889966063377317908>'),
  ('HOUSE_BALANCE', '<:balance:889966062962094090>'),
  ('EARLY_SUPPORTER', '<:discord_earlysupporter:971698655495082004>'),
  ('EARLY_VERIFIED_BOT_DEVELOPER', '<:verified:710970919736311942>'),
  ('DISCORD_CERTIFIED_MODERATOR', '<:certified_moderator:971699462072303656>');
