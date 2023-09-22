import { App, Stack, StackProps } from 'aws-cdk-lib';
import { LambdaConstruct } from '../constructs/lambda_construct';

export interface CoriolisApiStackProps extends StackProps {
    readonly prefix: string
    readonly isDev: boolean
}

export class CoriolisApiStack extends Stack {
    constructor(parent: App, name: string, props: CoriolisApiStackProps) {
        super(parent, name, props);

        const lambda_name = `${props.prefix}-coriolis-lambda-api`
        new LambdaConstruct(this, lambda_name, {
            functionName: lambda_name,
            packagePath: 'target/lambda/coriolis',
            isDev: props.isDev
        }).withFunctionUrl()
    }
}