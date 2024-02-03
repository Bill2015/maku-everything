import React, { SVGProps } from 'react';

const SvgrMock = React.forwardRef<SVGSVGElement, SVGProps<SVGSVGElement>>((props, ref) => (
    // eslint-disable-next-line react/jsx-props-no-spreading
    <svg ref={ref} {...props} />
));

SvgrMock.displayName = 'SvgrMock';

export const ReactComponent = SvgrMock;
export default SvgrMock;
