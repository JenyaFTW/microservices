FROM node:16-alpine

RUN mkdir -p /usr/src/nuxt-app
WORKDIR /usr/src/nuxt-app
COPY . .

RUN yarn install && yarn cache clean
RUN npm run build

ENV PORT=3004

EXPOSE 3004

ENTRYPOINT ["node", ".output/server/index.mjs"]