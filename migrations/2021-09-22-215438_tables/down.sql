-- This file should undo anything in `up.sql`
ALTER TABLE "moods" DROP CONSTRAINT IF EXISTS "moods_fk0";

ALTER TABLE "entrys" DROP CONSTRAINT IF EXISTS "entrys_fk0";

ALTER TABLE "entrys" DROP CONSTRAINT IF EXISTS "entrys_fk1";

ALTER TABLE "activities" DROP CONSTRAINT IF EXISTS "activities_fk0";

ALTER TABLE "entry_activities" DROP CONSTRAINT IF EXISTS "entry_activities_fk0";

ALTER TABLE "entry_activities" DROP CONSTRAINT IF EXISTS "entry_activities_fk1";

ALTER TABLE "entry_images" DROP CONSTRAINT IF EXISTS "entry_images_fk0";

ALTER TABLE "entry_images" DROP CONSTRAINT IF EXISTS "entry_images_fk1";

DROP TABLE "users";
DROP TABLE "moods";
DROP TABLE "entrys";
DROP TABLE "activities";
DROP TABLE "entry_activities";
DROP TABLE "entry_images";