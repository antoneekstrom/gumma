FROM node:20.5.1-alpine3.18

WORKDIR /gumma-groups

# Install dependencies
COPY package.json yarn.lock ./
RUN yarn --pure-lockfile --force

# Build
COPY . .
RUN yarn build

CMD ["yarn", "start"]