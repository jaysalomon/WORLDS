# POLIS System Specification

## Document Information
- **Version**: 1.0.0
- **Last Updated**: 2025-01-20
- **Status**: Draft
- **Author**: System Architect

---

## Table of Contents
1. [Executive Summary](#1-executive-summary)
2. [System Overview](#2-system-overview)
3. [Architecture](#3-architecture)
4. [Core Features](#4-core-features)
5. [Data Model](#5-data-model)
6. [API Specification](#6-api-specification)
7. [User Interface](#7-user-interface)
8. [Security](#8-security)
9. [Performance](#9-performance)
10. [Deployment](#10-deployment)

---

## 1. Executive Summary

POLIS is a comprehensive platform designed to facilitate democratic participation, civic engagement, and collective decision-making. The system enables structured conversations, opinion gathering, and consensus building through innovative digital mechanisms.

### Key Objectives
- Enable large-scale civic participation
- Facilitate structured deliberation
- Support evidence-based policy making
- Promote inclusive democratic processes
- Provide transparent decision-making tools

---

## 2. System Overview

### 2.1 Purpose
POLIS serves as a digital infrastructure for:
- **Public Consultation**: Gathering citizen input on policy matters
- **Deliberative Democracy**: Supporting structured public discourse
- **Consensus Building**: Identifying areas of agreement and disagreement
- **Policy Development**: Informing decision-makers with public sentiment

### 2.2 Target Users
| User Type | Description | Primary Use Case |
|-----------|-------------|------------------|
| Citizens | General public participants | Submit opinions, vote on statements |
| Moderators | Discussion facilitators | Manage conversations, ensure quality |
| Administrators | System operators | Configure discussions, analyze data |
| Policymakers | Decision makers | Review insights, inform policy |
| Researchers | Academic analysts | Study participation patterns |

### 2.3 Scope
- **In Scope**: Conversation management, opinion clustering, visualization, reporting
- **Out of Scope**: Identity verification, legal binding decisions, real-time chat

---

## 3. Architecture

### 3.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENT LAYER                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Web App   │  │  Mobile App │  │   Admin Dashboard   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      API GATEWAY LAYER                       │
│         (Authentication, Rate Limiting, Routing)             │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     SERVICE LAYER                            │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │
│  │ Conversation│ │  Voting  │ │  Report  │ │   Analytics  │   │
│  │   Service   │ │ Service  │ │ Service  │ │   Service    │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      DATA LAYER                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  PostgreSQL  │  │     Redis    │  │ Elasticsearch│     │
│  │  (Primary DB)│  │   (Cache)    │  │  (Search)    │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Frontend | React/TypeScript | User interface |
| Backend | Node.js/Python | API services |
| Database | PostgreSQL | Primary data storage |
| Cache | Redis | Session management, caching |
| Search | Elasticsearch | Full-text search |
| ML/AI | Python/scikit-learn | Opinion clustering |
| Queue | RabbitMQ/Redis | Background jobs |
| Monitoring | Prometheus/Grafana | System observability |

### 3.3 Deployment Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      LOAD BALANCER                           │
│                        (Nginx/ALB)                           │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼──────┐     ┌────────▼────────┐   ┌──────▼──────┐
│  App Server  │     │   App Server    │   │ App Server  │
│     #1      │     │      #2         │   │     #3      │
└─────────────┘     └─────────────────┘   └─────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    SHARED RESOURCES                          │
│         (Database, Cache, Storage, Message Queue)          │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. Core Features

### 4.1 Conversation Management

#### 4.1.1 Conversation Creation
- **Description**: Create new discussion topics with configurable parameters
- **Inputs**: Title, description, categories, moderation settings, visibility
- **Outputs**: Conversation ID, access URL, configuration summary
- **Constraints**: 
  - Title: 10-200 characters
  - Description: Up to 5000 characters
  - Max 10 categories per conversation

#### 4.1.2 Statement Submission
- **Description**: Participants submit opinions/statements for consideration
- **Inputs**: Statement text, participant ID, conversation ID
- **Validation**:
  - Length: 10-1000 characters
  - Prohibited content filtering
  - Duplicate detection
- **Process**: 
  1. Submit statement
  2. Automated moderation check
  3. Queue for review (if required)
  4. Publish or reject with reason

#### 4.1.3 Statement Moderation
- **Automated Checks**:
  - Spam detection
  - Profanity filtering
  - Duplicate detection
  - Length validation
- **Manual Review**:
  - Flagged content review
  - Appeal processing
  - Bulk operations

### 4.2 Voting System

#### 4.2.1 Vote Types
| Vote Type | Value | Description |
|-----------|-------|-------------|
| Agree | +1 | Strongly or somewhat agree |
| Disagree | -1 | Strongly or somewhat disagree |
| Pass | 0 | Uncertain or neutral |

#### 4.2.2 Voting Process
1. Participant views statement
2. Selects vote option
3. System records vote with timestamp
4. Real-time aggregation updates
5. Cluster analysis recalculation (async)

#### 4.2.3 Voting Constraints
- One vote per participant per statement
- Vote can be changed
- Pass votes don't affect clustering
- Minimum votes required for analysis: 10

### 4.3 Opinion Clustering

#### 4.3.1 Algorithm Overview
- **Method**: Principal Component Analysis (PCA) + K-means clustering
- **Input**: Vote matrix (participants × statements)
- **Output**: Opinion groups with characteristics

#### 4.3.2 Clustering Process
```
Raw Votes → Vote Matrix → PCA Dimensionality Reduction → 
K-means Clustering → Cluster Characterization → Visualization
```

#### 4.3.3 Cluster Characteristics
Each cluster includes:
- **Size**: Number of participants
- **Centroid**: Average position on key dimensions
- **Key Statements**: Most agreed/disagreed statements
- **Demographics**: Optional participant metadata
- **Consensus Areas**: Statements with high agreement
- **Disagreement Areas**: Statements with high variance

### 4.4 Visualization

#### 4.4.1 Opinion Map
- **Type**: 2D scatter plot
- **Axes**: Principal components from PCA
- **Points**: Individual participants
- **Colors**: Cluster membership
- **Interactivity**: Zoom, pan, hover for details

#### 4.4.2 Statement Analysis
- **Agreement Distribution**: Bar chart of vote counts
- **Cross-Cluster Comparison**: Heatmap of agreement by cluster
- **Consensus Ranking**: List of most/least agreed statements

#### 4.4.3 Participation Metrics
- **Timeline**: Participation over time
- **Demographics**: Optional participant breakdown
- **Engagement**: Votes per participant distribution

### 4.5 Reporting

#### 4.5.1 Standard Reports
1. **Executive Summary**: Key findings, consensus areas
2. **Detailed Analysis**: Full clustering results
3. **Participation Report**: Engagement metrics
4. **Raw Data Export**: CSV/JSON download

#### 4.5.2 Report Generation
- **Frequency**: On-demand, scheduled
- **Formats**: PDF, HTML, CSV, JSON
- **Distribution**: Email, download, API

---

## 5. Data Model

### 5.1 Entity Relationship Diagram

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Organization  │     │  Conversation   │     │    Statement    │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ PK id           │◄────┤ PK id           │◄────┤ PK id           │
│    name         │     │ FK org_id       │     │ FK conv_id      │
│    settings     │     │    title        │     │    text         │
│    created_at   │     │    description  │     │    status       │
└─────────────────┘     │    status       │     │    created_at   │
                        │    config       │     └─────────────────┘
                        │    created_at   │            │
                        └─────────────────┘            │
                               │                       │
                               │              ┌────────▼────────┐
                               │              │      Vote       │
                               │              ├─────────────────┤
                               │              │ PK id           │
                               │              │ FK stmt_id      │
                               │              │ FK part_id      │
                               │              │    value        │
                               │              │    created_at   │
                               │              └─────────────────┘
                               │                       ▲
                               │              ┌────────┘
                               │              │
                        ┌──────▼────────┐     │
                        │  Participant  │─────┘
                        ├─────────────────┤
                        │ PK id           │
                        │ FK conv_id      │
                        │    anon_id      │
                        │    metadata     │
                        │    created_at   │
                        └─────────────────┘
```

### 5.2 Core Entities

#### 5.2.1 Organization
```json
{
  "id": "uuid",
  "name": "string",
  "slug": "string",
  "settings": {
    "default_moderation": "boolean",
    "allow_anonymous": "boolean",
    "custom_branding": "object"
  },
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

#### 5.2.2 Conversation
```json
{
  "id": "uuid",
  "organization_id": "uuid",
  "title": "string",
  "description": "string",
  "status": "enum: draft|active|paused|closed",
  "config": {
    "moderation_mode": "enum: pre|post|none",
    "visibility": "enum: public|private|unlisted",
    "voting_enabled": "boolean",
    "statement_limit": "integer",
    "allowed_domains": ["string"]
  },
  "created_at": "timestamp",
  "updated_at": "timestamp",
  "closed_at": "timestamp|null"
}
```

#### 5.2.3 Statement
```json
{
  "id": "uuid",
  "conversation_id": "uuid",
  "text": "string",
  "status": "enum: pending|approved|rejected|flagged",
  "moderation_reason": "string|null",
  "submitted_by": "uuid",
  "moderated_by": "uuid|null",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

#### 5.2.4 Participant
```json
{
  "id": "uuid",
  "conversation_id": "uuid",
  "anonymous_id": "string",
  "metadata": {
    "demographics": "object|null",
    "source": "string",
    "referrer": "string"
  },
  "created_at": "timestamp",
  "last_active_at": "timestamp"
}
```

#### 5.2.5 Vote
```json
{
  "id": "uuid",
  "statement_id": "uuid",
  "participant_id": "uuid",
  "value": "integer: -1|0|1",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

### 5.3 Indexes

| Table | Index | Type | Purpose |
|-------|-------|------|---------|
| conversations | org_id + status | B-tree | List by organization |
| statements | conv_id + status | B-tree | Filter by conversation |
| votes | stmt_id + part_id | Unique | Prevent duplicate votes |
| votes | part_id + created_at | B-tree | Participant history |
| participants | conv_id + anon_id | Unique | Prevent duplicate participants |

---

## 6. API Specification

### 6.1 Authentication

#### 6.1.1 Methods
- **API Keys**: For server-to-server authentication
- **JWT Tokens**: For user session management
- **Anonymous Tokens**: For participant identification

#### 6.1.2 Token Format
```
Authorization: Bearer <token>
```

### 6.2 Endpoints

#### 6.2.1 Conversations

**List Conversations**
```
GET /api/v1/conversations
```
Parameters:
- `organization_id` (optional): Filter by organization
- `status` (optional): Filter by status
- `page` (optional): Pagination page
- `per_page` (optional): Items per page (max 100)

Response:
```json
{
  "data": [
    {
      "id": "uuid",
      "title": "string",
      "status": "string",
      "participant_count": "integer",
      "statement_count": "integer",
      "created_at": "timestamp"
    }
  ],
  "meta": {
    "page": "integer",
    "per_page": "integer",
    "total": "integer",
    "total_pages": "integer"
  }
}
```

**Create Conversation**
```
POST /api/v1/conversations
```
Request:
```json
{
  "title": "string (required, 10-200 chars)",
  "description": "string (optional, max 5000 chars)",
  "config": {
    "moderation_mode": "pre|post|none",
    "visibility": "public|private|unlisted"
  }
}
```

**Get Conversation**
```
GET /api/v1/conversations/{id}
```

**Update Conversation**
```
PATCH /api/v1/conversations/{id}
```

**Delete Conversation**
```
DELETE /api/v1/conversations/{id}
```

#### 6.2.2 Statements

**List Statements**
```
GET /api/v1/conversations/{conversation_id}/statements
```
Parameters:
- `status` (optional): Filter by status
- `sort` (optional): created_at|agreement_count|vote_count

**Submit Statement**
```
POST /api/v1/conversations/{conversation_id}/statements
```
Request:
```json
{
  "text": "string (required, 10-1000 chars)"
}
```

**Moderate Statement**
```
PATCH /api/v1/statements/{id}
```
Request:
```json
{
  "status": "approved|rejected",
  "reason": "string (required if rejected)"
}
```

#### 6.2.3 Voting

**Cast Vote**
```
POST /api/v1/statements/{statement_id}/votes
```
Request:
```json
{
  "value": "integer: -1|0|1"
}
```

**Get Vote Results**
```
GET /api/v1/conversations/{conversation_id}/results
```
Response:
```json
{
  "conversation_id": "uuid",
  "total_participants": "integer",
  "total_votes": "integer",
  "clusters": [
    {
      "id": "integer",
      "size": "integer",
      "centroid": ["float"],
      "key_statements": ["uuid"],
      "agreement_distribution": {
        "agree": "float",
        "disagree": "float",
        "pass": "float"
      }
    }
  ],
  "statements": [
    {
      "id": "uuid",
      "text": "string",
      "votes": {
        "agree": "integer",
        "disagree": "integer",
        "pass": "integer"
      },
      "consensus_score": "float"
    }
  ]
}
```

#### 6.2.4 Participants

**Join Conversation**
```
POST /api/v1/conversations/{conversation_id}/participants
```
Response:
```json
{
  "participant_id": "uuid",
  "anonymous_id": "string",
  "token": "string"
}
```

**Get Participant Votes**
```
GET /api/v1/participants/{id}/votes
```

### 6.3 Error Handling

#### 6.3.1 Error Format
```json
{
  "error": {
    "code": "string",
    "message": "string",
    "details": "object|null"
  }
}
```

#### 6.3.2 HTTP Status Codes
| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | Successful GET, PATCH |
| 201 | Created | Successful POST |
| 204 | No Content | Successful DELETE |
| 400 | Bad Request | Invalid input |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource doesn't exist |
| 409 | Conflict | Resource already exists |
| 422 | Unprocessable | Validation failed |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Server Error | Internal error |

### 6.4 Rate Limiting

| Endpoint Type | Limit | Window |
|---------------|-------|--------|
| General API | 1000 | 1 hour |
| Statement submission | 10 | 1 minute |
| Voting | 60 | 1 minute |
| Authentication | 10 | 1 minute |

---

## 7. User Interface

### 7.1 Public Interface

#### 7.1.1 Conversation Page
**Layout**:
```
┌─────────────────────────────────────────────────────────────┐
│  Header: Logo, Navigation, Language Selector                │
├─────────────────────────────────────────────────────────────┤
│  Title: [Conversation Title]                                 │
│  Description: [Conversation Description]                     │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐  ┌─────────────────────────────┐  │
│  │                     │  │                             │  │
│  │   Statement Card    │  │      Opinion Map            │  │
│  │   - Text            │  │      (Visualization)        │  │
│  │   - Vote buttons    │  │                             │  │
│  │   - Results         │  │                             │  │
│  │                     │  │                             │  │
│  └─────────────────────┘  └─────────────────────────────┘  │
│                                                           │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              Submit New Statement                    │  │
│  │  [Text Area] [Submit Button]                        │  │
│  └─────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  Footer: About, Privacy, Terms, Help                       │
└─────────────────────────────────────────────────────────────┘
```

**Components**:
- **Statement Card**: Displays statement with voting options
- **Opinion Map**: Interactive visualization of participant positions
- **Submission Form**: Text area with character counter
- **Progress Indicator**: Shows participation progress

#### 7.1.2 Results Page
**Sections**:
1. **Overview**: Summary statistics
2. **Opinion Map**: Cluster visualization
3. **Statement Analysis**: Agreement rankings
4. **Cluster Details**: Group characteristics

### 7.2 Admin Interface

#### 7.2.1 Dashboard
**Widgets**:
- Active conversations count
- Recent activity feed
- Moderation queue status
- System health metrics

#### 7.2.2 Conversation Management
**Features**:
- Create/edit conversations
- Configure settings
- Manage participants
- View analytics
- Export data

#### 7.2.3 Moderation Interface
**Features**:
- Review queue
- Bulk actions
- Filter and search
- Moderation history
- Appeal handling

### 7.3 Responsive Design

#### 7.3.1 Breakpoints
| Breakpoint | Width | Layout Adjustments |
|------------|-------|-------------------|
| Mobile | < 640px | Single column, stacked |
| Tablet | 640-1024px | Two columns where applicable |
| Desktop | > 1024px | Full multi-column layout |

#### 7.3.2 Mobile Considerations
- Touch-friendly vote buttons
- Simplified visualizations
- Bottom navigation
- Optimized text input

---

## 8. Security

### 8.1 Authentication & Authorization

#### 8.1.1 User Roles
| Role | Permissions |
|------|-------------|
| Anonymous Participant | Vote, submit statements (if allowed) |
| Authenticated User | All participant actions + history |
| Moderator | Review content, manage participants |
| Admin | Full conversation management |
| Super Admin | System-wide configuration |

#### 8.1.2 Access Control
- Role-based access control (RBAC)
- Resource-level permissions
- Organization isolation
- API key scopes

### 8.2 Data Protection

#### 8.2.1 Encryption
- **In Transit**: TLS 1.3
- **At Rest**: AES-256
- **Database**: Transparent Data Encryption (TDE)

#### 8.2.2 PII Handling
- Minimal data collection
- Anonymization of participant data
- Data retention policies
- Right to deletion

### 8.3 Security Measures

#### 8.3.1 Input Validation
- SQL injection prevention
- XSS protection
- CSRF tokens
- Content Security Policy

#### 8.3.2 Abuse Prevention
- Rate limiting
- CAPTCHA for submissions
- Automated spam detection
- IP-based restrictions

#### 8.3.3 Audit Logging
- Authentication events
- Administrative actions
- Data access logs
- Retention: 90 days

---

## 9. Performance

### 9.1 Performance Requirements

| Metric | Target | Critical Threshold |
|--------|--------|-------------------|
| Page Load Time | < 2s | < 5s |
| API Response Time | < 200ms | < 1s |
| Vote Processing | < 100ms | < 500ms |
| Clustering Update | < 5 minutes | < 15 minutes |
| Availability | 99.9% | 99.5% |

### 9.2 Optimization Strategies

#### 9.2.1 Caching
- **Redis**: Session data, frequent queries
- **CDN**: Static assets, visualization data
- **Application Cache**: Computed results

#### 9.2.2 Database Optimization
- Query optimization
- Connection pooling
- Read replicas for analytics
- Partitioning for large tables

#### 9.2.3 Async Processing
- Vote aggregation
- Clustering calculations
- Report generation
- Email notifications

### 9.3 Scalability

#### 9.3.1 Horizontal Scaling
- Stateless application servers
- Shared session storage
- Database read replicas
- Load balancer configuration

#### 9.3.2 Capacity Planning
| Resource | Baseline | Peak Load |
|----------|----------|-----------|
| Concurrent Users | 1,000 | 10,000 |
| Votes/Second | 100 | 1,000 |
| Statements/Minute | 50 | 500 |
| Data Growth | 1GB/month | 10GB/month |

---

## 10. Deployment

### 10.1 Infrastructure Requirements

#### 10.1.1 Production Environment
| Component | Specification | Quantity |
|-------------|-------------|----------|
| Application Server | 4 vCPU, 8GB RAM | 3+ |
| Database Server | 8 vCPU, 32GB RAM | 2 (primary + replica) |
| Cache Server | 2 vCPU, 8GB RAM | 2 (clustered) |
| Search Server | 4 vCPU, 16GB RAM | 3 (cluster) |
| Load Balancer | Managed service | 1 |
| Storage | SSD, 500GB | Shared |

#### 10.1.2 Development Environment
| Component | Specification |
|-----------|-------------|
| Application Server | 2 vCPU, 4GB RAM |
| Database | Shared instance |
| Cache | Shared instance |
| Storage | 50GB |

### 10.2 Deployment Process

#### 10.2.1 CI/CD Pipeline
```
Code Commit → Build → Test → Security Scan → 
Deploy to Staging → Integration Tests → 
Deploy to Production → Health Check → Monitor
```

#### 10.2.2 Deployment Steps
1. **Pre-deployment**:
   - Run automated tests
   - Database migration check
   - Security scan
   - Backup verification

2. **Deployment**:
   - Deploy to canary (5% traffic)
   - Monitor for 30 minutes
   - Gradual rollout (25% → 50% → 100%)
   - Database migrations

3. **Post-deployment**:
   - Health checks
   - Smoke tests
   - Performance monitoring
   - Rollback plan ready

### 10.3 Monitoring & Alerting

#### 10.3.1 Metrics
- Application performance (response times, error rates)
- Infrastructure (CPU, memory, disk)
- Business metrics (active users, votes)
- Security events

#### 10.3.2 Alerting Thresholds
| Metric | Warning | Critical |
|--------|---------|----------|
| Error Rate | > 1% | > 5% |
| Response Time | > 500ms | > 2s |
| CPU Usage | > 70% | > 90% |
| Memory Usage | > 80% | > 95% |
| Disk Usage | > 80% | > 95% |

### 10.4 Disaster Recovery

#### 10.4.1 Backup Strategy
- **Database**: Continuous replication + daily snapshots
- **File Storage**: Versioned backups
- **Configuration**: Infrastructure as code

#### 10.4.2 Recovery Objectives
- **RPO** (Recovery Point Objective): 1 hour
- **RTO** (Recovery Time Objective): 4 hours

#### 10.4.3 Failover Procedures
1. Database failover to replica
2. Traffic rerouting to healthy regions
3. Notification to stakeholders
4. Post-incident review

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| Cluster | Group of participants with similar voting patterns |
| Consensus | High level of agreement across all participants |
| Conversation | A structured discussion topic |
| PCA | Principal Component Analysis, dimensionality reduction technique |
| Statement | An opinion or assertion submitted by a participant |
| Vote | Participant's response to a statement (agree/disagree/pass) |

## Appendix B: References

- [Pol.is](https://pol.is/) - Original platform
- [Deliberative Democracy](https://en.wikipedia.org/wiki/Deliberative_democracy)
- [Participatory Democracy](https://en.wikipedia.org/wiki/Participatory_democracy)

## Appendix C: Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2025-01-20 | System Architect | Initial specification |

---

*End of Document*
