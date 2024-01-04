export const handler = async (event) => {
  const response = {
    statusCode: 200,
    body: JSON.stringify('the square of input is '+ (event.command * event.command)),
  };
  return response;
};
