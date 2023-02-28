FROM node:17-alpine AS base

WORKDIR /opt/app
COPY package*.json .
RUN npm install
COPY . .

FROM base AS builder

RUN npm exec prisma generate
RUN npm run build

FROM builder as development

RUN apk add git
CMD npm run dev

FROM builder as production

CMD npm run start
