import { App } from 'aws-cdk-lib';
import { CoriolisApiStack } from './lib/stacks/coriolis_api_stack';
import { ENVIRONMENTS } from './lib/config/environments';

const app = new App();

ENVIRONMENTS.forEach(environment => {
    new CoriolisApiStack(app, `${environment.name}-coriolis-api-stack`, {
        env: {
            region: environment.region,
            account: environment.account
        },
        prefix: environment.name,
        isDev: environment.isDev
    });
})

app.synth();