import { getYoutubeVideoId } from './urlParser';

describe('getYoutubeVideoId', () => {
    it.each([
        // normal url
        {
            inputURL: 'https://www.youtube.com/watch?v=uWxGMzSdX0c',
            expectId: 'uWxGMzSdX0c',
        },
        {
            inputURL: 'https://www.youtube.com/watch?v=uWxGMzSdX0c&list=RDuWxGMzSdX0c&start_radio=1',
            expectId: 'uWxGMzSdX0c',
        },
        {
            inputURL: 'https://www.youtube.com/watch?v=uWxGMzSdX0c?list=RDuWxGMzSdX0c',
            expectId: 'uWxGMzSdX0c',
        },
        // shorts
        {
            inputURL: 'https://www.youtube.com/shorts/7IxS5Y2eMw0',
            expectId: '7IxS5Y2eMw0',
        },
        {
            inputURL: 'https://www.youtube.com/shorts/7IxS5Y2eMw0&list=RDuWxGMzSdX0c&',
            expectId: '7IxS5Y2eMw0',
        },
    ])('Should get "$expectId" youtube id when input Youtube URL: "$inputURL"', ({ inputURL, expectId }) => {
        // Arrange

        // Act
        const result = getYoutubeVideoId(inputURL);

        // Assert
        expect(result).toEqual(expectId);
    });

    it.each([
        // normal
        'https://wwwabc.youtube.com/watch?v=uWxGMzSdX0c',
        'https://www.youtube.com/watch?vaaaaa=uWxGMzSdX0c',
        'https://www.youtube.com/watch?v=',
        // shorts
        'https://www.youtube.com/longs/7IxS5Y2eMw0',
        'https://www.youtube.com/shorts/',
    ])('Should return null when input Youtube URL: "%s"', (val) => {
        // Arrange

        // Act
        const result = getYoutubeVideoId(val);

        // Assert
        expect(result).toBeNull();
    });
});
