CREATE TABLE IF NOT EXISTS "insights" (
                                          "insight_id" uuid PRIMARY KEY NOT NULL,
                                          "event_id" uuid NOT NULL,
                                          "insight" text NOT NULL,
                                          "created_at" timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS  "insights_events" (
                                                  "event_id" uuid PRIMARY KEY NOT NULL,
                                                  "event_subject" text NOT NULL,
                                                  "club_id" int8 NOT NULL,
                                                  "filling" boolean NOT NULL DEFAULT true,
                                                  "finished" boolean NOT NULL DEFAULT false,
                                                  "created_at" timestamptz NOT NULL DEFAULT NOW(),
                                                  "started_at" timestamptz,
                                                  "finished_at" timestamptz
)