# WORLDS Platform Database Schema

## Overview

This document defines the database schema for the WORLDS platform. We use a polyglot persistence approach with multiple databases optimized for different data types and access patterns.

## PostgreSQL Schema (Primary Database)

### Users & Authentication

```sql
-- Core user table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(32) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(64),
    avatar_url TEXT,
    bio TEXT,
    status user_status DEFAULT 'active',
    email_verified BOOLEAN DEFAULT FALSE,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(255),
    last_login_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- User roles for RBAC
CREATE TABLE user_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role role_type NOT NULL,
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    granted_by UUID REFERENCES users(id),
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(user_id, role, world_id)
);

-- User sessions
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User preferences
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    notification_settings JSONB DEFAULT '{}',
    privacy_settings JSONB DEFAULT '{}',
    ui_settings JSONB DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Worlds & Instances

```sql
-- World definitions
CREATE TABLE worlds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id),
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) UNIQUE NOT NULL,
    description TEXT,
    category world_category,
    status world_status DEFAULT 'draft',
    visibility visibility_type DEFAULT 'public',
    max_players INTEGER DEFAULT 100,
    world_data JSONB NOT NULL DEFAULT '{}',
    thumbnail_url TEXT,
    banner_url TEXT,
    tags TEXT[],
    rating DECIMAL(2,1) CHECK (rating >= 0 AND rating <= 5),
    rating_count INTEGER DEFAULT 0,
    visit_count BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    published_at TIMESTAMP WITH TIME ZONE
);

-- World instances (running copies)
CREATE TABLE world_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    instance_type instance_type DEFAULT 'shared',
    status instance_status DEFAULT 'initializing',
    server_node VARCHAR(255),
    region VARCHAR(64),
    current_players INTEGER DEFAULT 0,
    max_players INTEGER DEFAULT 100,
    metadata JSONB DEFAULT '{}',
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended_at TIMESTAMP WITH TIME ZONE
);

-- World memberships
CREATE TABLE world_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    role membership_role DEFAULT 'visitor',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_visited_at TIMESTAMP WITH TIME ZONE,
    visit_count INTEGER DEFAULT 0,
    UNIQUE(user_id, world_id)
);

-- World bans
CREATE TABLE world_bans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    banned_by UUID REFERENCES users(id),
    reason TEXT,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(world_id, user_id)
);
```

### Economy & Assets

```sql
-- Currencies
CREATE TABLE currencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    name VARCHAR(64) NOT NULL,
    symbol VARCHAR(8) NOT NULL,
    decimals INTEGER DEFAULT 2,
    max_supply BIGINT,
    current_supply BIGINT DEFAULT 0,
    blockchain_address VARCHAR(42),
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User wallets
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    currency_id UUID REFERENCES currencies(id),
    balance BIGINT DEFAULT 0,
    blockchain_address VARCHAR(42),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, world_id, currency_id)
);

-- Transactions
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id),
    from_wallet_id UUID REFERENCES wallets(id),
    to_wallet_id UUID REFERENCES wallets(id),
    currency_id UUID REFERENCES currencies(id),
    amount BIGINT NOT NULL,
    transaction_type transaction_type NOT NULL,
    status transaction_status DEFAULT 'pending',
    metadata JSONB DEFAULT '{}',
    blockchain_tx_hash VARCHAR(66),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    confirmed_at TIMESTAMP WITH TIME ZONE
);

-- Assets (NFTs)
CREATE TABLE assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    owner_id UUID REFERENCES users(id),
    creator_id UUID REFERENCES users(id),
    asset_type asset_type NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    content_url TEXT,
    thumbnail_url TEXT,
    blockchain_token_id VARCHAR(255),
    blockchain_contract VARCHAR(42),
    is_minted BOOLEAN DEFAULT FALSE,
    is_listed BOOLEAN DEFAULT FALSE,
    list_price BIGINT,
    list_currency_id UUID REFERENCES currencies(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Asset transfers
CREATE TABLE asset_transfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID REFERENCES assets(id) ON DELETE CASCADE,
    from_user_id UUID REFERENCES users(id),
    to_user_id UUID REFERENCES users(id),
    price BIGINT,
    currency_id UUID REFERENCES currencies(id),
    transfer_type transfer_type NOT NULL,
    blockchain_tx_hash VARCHAR(66),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Social Features

```sql
-- Friendships
CREATE TABLE friendships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    requester_id UUID REFERENCES users(id) ON DELETE CASCADE,
    addressee_id UUID REFERENCES users(id) ON DELETE CASCADE,
    status friendship_status DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(requester_id, addressee_id)
);

-- Groups/Guilds
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) NOT NULL,
    description TEXT,
    avatar_url TEXT,
    banner_url TEXT,
    is_public BOOLEAN DEFAULT TRUE,
    max_members INTEGER DEFAULT 100,
    member_count INTEGER DEFAULT 1,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(world_id, slug)
);

-- Group memberships
CREATE TABLE group_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID REFERENCES groups(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role group_role DEFAULT 'member',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(group_id, user_id)
);

-- Messages
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID REFERENCES users(id) ON DELETE CASCADE,
    recipient_id UUID REFERENCES users(id) ON DELETE CASCADE,
    world_id UUID REFERENCES worlds(id),
    group_id UUID REFERENCES groups(id),
    message_type message_type DEFAULT 'text',
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    is_read BOOLEAN DEFAULT FALSE,
    read_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### NPCs & AI

```sql
-- NPC definitions
CREATE TABLE npcs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    world_id UUID REFERENCES worlds(id) ON DELETE CASCADE,
    creator_id UUID REFERENCES users(id),
    name VARCHAR(128) NOT NULL,
    description TEXT,
    personality JSONB NOT NULL DEFAULT '{}',
    knowledge_base JSONB DEFAULT '{}',
    behavior_config JSONB DEFAULT '{}',
    avatar_config JSONB DEFAULT '{}',
    voice_config JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    spawn_location JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- NPC memories
CREATE TABLE npc_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    npc_id UUID REFERENCES npcs(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    memory_type memory_type NOT NULL,
    content TEXT NOT NULL,
    importance_score DECIMAL(3,2) DEFAULT 0.5,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- NPC conversations
CREATE TABLE npc_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    npc_id UUID REFERENCES npcs(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    world_instance_id UUID REFERENCES world_instances(id),
    context JSONB DEFAULT '{}',
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ended_at TIMESTAMP WITH TIME ZONE
);

-- Conversation messages
CREATE TABLE npc_conversation_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID REFERENCES npc_conversations(id) ON DELETE CASCADE,
    is_from_npc BOOLEAN NOT NULL,
    content TEXT NOT NULL,
    emotion VARCHAR(32),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## MongoDB Schema (Content & Events)

### Collections

```javascript
// User-generated content
{
  _id: ObjectId,
  worldId: UUID,
  creatorId: UUID,
  contentType: "script|blueprint|texture|audio|animation",
  name: String,
  description: String,
  version: Number,
  content: Object, // Flexible schema based on type
  dependencies: [UUID],
  tags: [String],
  ratings: {
    average: Number,
    count: Number
  },
  downloadCount: Number,
  createdAt: ISODate,
  updatedAt: ISODate
}

// World events/activities
{
  _id: ObjectId,
  worldId: UUID,
  instanceId: UUID,
  eventType: "player_join|player_leave|chat|action|system",
  actorId: UUID, // user or npc
  targetId: UUID, // optional
  position: {
    x: Number,
    y: Number,
    z: Number
  },
  data: Object, // Event-specific data
  timestamp: ISODate
}

// World snapshots
{
  _id: ObjectId,
  worldId: UUID,
  instanceId: UUID,
  snapshotData: Object, // Compressed world state
  entityCount: Number,
  playerCount: Number,
  createdAt: ISODate,
  expiresAt: ISODate
}
```

## Redis Data Structures

### Key Patterns

```
# Session management
session:{token} → user_id, expires_at
user_sessions:{user_id} → [token1, token2, ...]

# World state (per instance)
world:{instance_id}:players → Hash {user_id: position_data}
world:{instance_id}:entities → Hash {entity_id: entity_data}
world:{instance_id}:chat → Stream (last 100 messages)

# Rate limiting
ratelimit:{user_id}:{action} → Counter with TTL

# Caching
cache:api:{endpoint_hash} → JSON response
user:{user_id}:profile → Cached profile data
world:{world_id}:info → Cached world info

# Real-time presence
presence:{world_id} → Set of online user IDs
user:{user_id}:presence → {world_id, instance_id, last_seen}

# Leaderboards
leaderboard:{world_id}:{category} → Sorted Set
```

## TimescaleDB Schema (Time-Series Data)

### Player Metrics

```sql
-- Player positions (hypertable)
CREATE TABLE player_positions (
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    user_id UUID NOT NULL,
    world_id UUID NOT NULL,
    instance_id UUID NOT NULL,
    position_x DECIMAL(10, 3),
    position_y DECIMAL(10, 3),
    position_z DECIMAL(10, 3),
    rotation_x DECIMAL(5, 3),
    rotation_y DECIMAL(5, 3),
    rotation_z DECIMAL(5, 3)
);

SELECT create_hypertable('player_positions', 'time', chunk_time_interval => INTERVAL '1 hour');

-- Player sessions
CREATE TABLE player_sessions (
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    user_id UUID NOT NULL,
    world_id UUID NOT NULL,
    instance_id UUID NOT NULL,
    session_id UUID NOT NULL,
    event_type session_event NOT NULL,
    duration_seconds INTEGER
);

SELECT create_hypertable('player_sessions', 'time', chunk_time_interval => INTERVAL '1 day');

-- Economy metrics
CREATE TABLE economy_metrics (
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    world_id UUID NOT NULL,
    currency_id UUID NOT NULL,
    metric_type metric_type NOT NULL,
    value BIGINT NOT NULL
);

SELECT create_hypertable('economy_metrics', 'time', chunk_time_interval => INTERVAL '1 day');
```

## Elasticsearch Indices

### Searchable Content

```json
{
  "mappings": {
    "properties": {
      "id": { "type": "keyword" },
      "type": { "type": "keyword" },
      "world_id": { "type": "keyword" },
      "title": { 
        "type": "text",
        "analyzer": "standard"
      },
      "description": { 
        "type": "text",
        "analyzer": "standard"
      },
      "tags": { "type": "keyword" },
      "creator": {
        "properties": {
          "id": { "type": "keyword" },
          "username": { "type": "keyword" }
        }
      },
      "rating": { "type": "float" },
      "created_at": { "type": "date" },
      "popularity_score": { "type": "float" }
    }
  }
}
```

## Indexing Strategy

### PostgreSQL Indexes

```sql
-- Users
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_status ON users(status) WHERE deleted_at IS NULL;

-- Worlds
CREATE INDEX idx_worlds_slug ON worlds(slug);
CREATE INDEX idx_worlds_owner ON worlds(owner_id);
CREATE INDEX idx_worlds_category ON worlds(category) WHERE status = 'published';
CREATE INDEX idx_worlds_rating ON worlds(rating DESC) WHERE status = 'published';
CREATE INDEX idx_worlds_search ON worlds USING gin(to_tsvector('english', name || ' ' || COALESCE(description, '')));

-- World instances
CREATE INDEX idx_instances_world ON world_instances(world_id);
CREATE INDEX idx_instances_status ON world_instances(status);

-- Transactions
CREATE INDEX idx_transactions_from ON transactions(from_wallet_id, created_at DESC);
CREATE INDEX idx_transactions_to ON transactions(to_wallet_id, created_at DESC);
CREATE INDEX idx_transactions_world ON transactions(world_id, created_at DESC);

-- Messages
CREATE INDEX idx_messages_sender ON messages(sender_id, created_at DESC);
CREATE INDEX idx_messages_recipient ON messages(recipient_id, created_at DESC);
CREATE INDEX idx_messages_world ON messages(world_id, created_at DESC) WHERE world_id IS NOT NULL;

-- NPC memories
CREATE INDEX idx_memories_npc ON npc_memories(npc_id, importance_score DESC);
CREATE INDEX idx_memories_user ON npc_memories(user_id);
CREATE INDEX idx_memories_expiry ON npc_memories(expires_at) WHERE expires_at IS NOT NULL;
```

## Data Retention Policies

| Data Type | Retention Period | Archive Strategy |
|-----------|-----------------|------------------|
| User data | Indefinite | Soft delete only |
| World data | Indefinite | Versioned |
| Chat messages | 90 days | Archive to S3 |
| Player positions | 30 days | Aggregate only |
| System logs | 30 days | Archive to S3 |
| NPC conversations | 30 days | Anonymize & archive |
| Transaction history | 7 years | Immutable |
| World snapshots | 7 days | Keep last 10 per world |

## Migration Strategy

### Version Control
- All schema changes tracked in `/migrations`
- Naming: `YYYYMMDDHHMMSS_description.sql`
- Rollback scripts required for each migration

### Zero-Downtime Migrations
1. Add new column/table (nullable/default)
2. Deploy code using new schema
3. Backfill data if needed
4. Make column NOT NULL if required
5. Remove old column/table

## Backup & Recovery

### PostgreSQL
- Continuous WAL archiving to S3
- Daily full backups
- Point-in-time recovery capability

### MongoDB
- Replica set with 3 nodes
- Daily snapshots
- Oplog-based incremental backup

### Redis
- RDB snapshots every 6 hours
- AOF persistence enabled
- Replication to standby
