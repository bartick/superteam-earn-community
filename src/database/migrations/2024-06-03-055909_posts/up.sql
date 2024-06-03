-- Your SQL goes here
CREATE TABLE posts (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    title TEXT,
    slug TEXT,
    deadline TIMESTAMP,
    token TEXT,
    rewardAmount INT,
    rewards JSONB,
    skills JSONB[],
    _type TEXT,
    requirements TEXT,
    totalPaymentsMade INT DEFAULT 0,
    totalWinnersSelected INT DEFAULT 0,
    isWinnersAnnounced BOOLEAN DEFAULT FALSE,
    region TEXT,
    pocSocials TEXT,
    hackathonprize BOOLEAN DEFAULT FALSE,
    -- check properly
    timeToComplete TEXT,
    winners JSONB,
    sponsor JSONB
);

