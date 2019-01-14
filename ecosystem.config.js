module.exports = {
    apps : [{
        name: 'botium_sample_rivescript',
        script: 'npm',
        args: [ 'start' ],
        instances: 1,
        autorestart: true,
        env: {
            PORT: 3002
        }
    }],
};