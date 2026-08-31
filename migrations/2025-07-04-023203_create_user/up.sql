CREATE SCHEMA IF NOT EXISTS "user";

CREATE TABLE IF NOT EXISTS "user"."users" (
  "created_at" TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "status" TEXT NOT NULL,
    "id" BIGSERIAL NOT NULL,
    "uuid" UUID NOT NULL DEFAULT gen_random_uuid (),
    "email" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NULL,
    "email_verified_at" TIMESTAMP
  WITH
    TIME ZONE NULL,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS users_uuid_key ON "user"."users" ("uuid");
CREATE UNIQUE INDEX IF NOT EXISTS users_email_key ON "user"."users" ("email");

CREATE TABLE IF NOT EXISTS "user"."users_identities" (
  "created_at" TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP
  WITH
    TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "id" BIGSERIAL NOT NULL,
    "uuid" UUID NOT NULL DEFAULT gen_random_uuid (),
    "subject" TEXT NOT NULL,
    "provider" TEXT NOT NULL,
    "user_id" BIGSERIAL NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "user"."users" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS users_identities_user_provider_key ON "user"."users_identities" ("user_id", "provider");
CREATE UNIQUE INDEX IF NOT EXISTS users_identities_provider_subject_key ON "user"."users_identities" ("provider", "subject");
