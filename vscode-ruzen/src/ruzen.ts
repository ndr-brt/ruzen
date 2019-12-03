import udp = require('dgram');

export class Ruzen {
    client: udp.Socket = udp.createSocket('udp4');

    constructor() { }

    public eval(code:string) {
        this.client.send(code, 38043, 'localhost', function(error: any) {
            if (error) {
                console.error(`Error ${error}`);
            } else {
                console.log('Data sent:');
                console.log(code);
            }
        });
    }

}