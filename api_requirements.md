ONLY POLL A ROUTE IF IT EXPLICITLY STATES THAT POLLING IS PERMITTED
API IS UNSECURED AT THIS POINT IN TIME

Required API routes (without authorization):
- ~~GET the latest news, one of each source `/news/latest`~~
- ~~GET specific sources latest news `/news/latest/{source}`~~
- ~~GET the latest date of news + source where it was fetched `/news/timestamp` (Polling allowed)~~
- GET statistics-struct `/statistics`
- ~~GET uptime `/statistics/uptime`~~
 
Required API routes (with authorization):
- POST manually send news `/news/post`
- ~~POST remote-shutdown `/settings/shutdown`~~
- POST && GET edit-webhook filters `/settings/webhooks/filters` (User-auth based view of filters)
- GET dump warning logfile `/log/warning`
- GET dump debug logfile `/log/debug`
- GET && POST time-out map `/timeout`
- GET && POST change-default-keywords `/settings/webhooks/filters-default`