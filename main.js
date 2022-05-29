import './node_modules/@patternfly/patternfly/patternfly.scss';
import './node_modules/@patternfly/patternfly/patternfly-addons.scss';

import("./pkg").then(module => {
    module.main();
});