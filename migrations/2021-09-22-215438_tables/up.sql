-- Your SQL goes here
CREATE TABLE "users" (
	"id" SERIAL NOT NULL,
	"email" VARCHAR(255) NOT NULL UNIQUE,
	"hash" VARCHAR(255) NOT NULL,
	CONSTRAINT "users_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);
CREATE TABLE "moods" (
	"id" SERIAL NOT NULL,
	"user_id" INT NOT NULL,
	"name" TEXT NOT NULL,
	"value" INT NOT NULL,
	"icon" CHARACTER NOT NULL,
	CONSTRAINT "moods_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);
CREATE TABLE "entrys" (
	"id" SERIAL NOT NULL,
	"user_id" INT NOT NULL,
	"mood_id" INT NOT NULL,
	"desc" TEXT,
	"created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
	CONSTRAINT "entrys_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);
CREATE TABLE "activities" (
	"id" SERIAL NOT NULL,
	"user_id" INT NOT NULL,
	"name" TEXT NOT NULL,
	"icon" CHARACTER NOT NULL,
	CONSTRAINT "activities_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);
CREATE TABLE "entry_activities" (
	"id" serial NOT NULL,
	"entry_id" int NOT NULL,
	"activity_id" int NOT NULL,
	CONSTRAINT "entry_activities_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);
CREATE TABLE "entry_images" (
	"id" serial NOT NULL,
	"user_id" int NOT NULL,
	"entry_id" int NOT NULL,
	"image_url" TEXT NOT NULL,
	CONSTRAINT "entry_images_pk" PRIMARY KEY ("id")
) WITH (OIDS = FALSE);
ALTER TABLE "moods"
ADD CONSTRAINT "moods_fk0" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE;
ALTER TABLE "entrys"
ADD CONSTRAINT "entrys_fk0" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE;
ALTER TABLE "entrys"
ADD CONSTRAINT "entrys_fk1" FOREIGN KEY ("mood_id") REFERENCES "moods"("id") ON DELETE CASCADE;
ALTER TABLE "activities"
ADD CONSTRAINT "activities_fk0" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE;
ALTER TABLE "entry_activities"
ADD CONSTRAINT "entry_activities_fk0" FOREIGN KEY ("entry_id") REFERENCES "entrys"("id") ON DELETE CASCADE;
ALTER TABLE "entry_activities"
ADD CONSTRAINT "entry_activities_fk1" FOREIGN KEY ("activity_id") REFERENCES "activities"("id") ON DELETE CASCADE;
ALTER TABLE "entry_images"
ADD CONSTRAINT "entry_images_fk0" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE;
ALTER TABLE "entry_images"
ADD CONSTRAINT "entry_images_fk1" FOREIGN KEY ("entry_id") REFERENCES "entrys"("id") ON DELETE CASCADE;
