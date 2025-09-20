CREATE TABLE IF NOT EXISTS "user"."persons" (
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "status" TEXT NOT NULL,
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "first_name" VARCHAR(150) NULL,
    "last_name" VARCHAR(250) NULL,
    "document_number" VARCHAR(12) NULL,
    "document_type" VARCHAR(50) NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "user"."users" (
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "status" TEXT NOT NULL,
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(50) NOT NULL,
    "person_id" UUID NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("person_id") REFERENCES "user"."persons" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS "user"."cellphones" (
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "status" TEXT NOT NULL,
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "extension" VARCHAR(6) NOT NULL DEFAULT '+591',
    "number" INT NOT NULL,
    "user_id" UUID NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "user"."users" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
