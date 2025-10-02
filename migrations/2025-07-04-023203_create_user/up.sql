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
    PRIMARY KEY ("id")
);

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
